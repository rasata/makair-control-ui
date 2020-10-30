// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub mod settings;

use chrono::{offset::Utc, DateTime, Duration};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::sync::mpsc::{self, Receiver, Sender};

use settings::{trigger::SettingsTriggerState, ChipSettings, ChipSettingsEvent};
use telemetry::alarm::{AlarmCode, RMC_SW_1, RMC_SW_11, RMC_SW_12, RMC_SW_14, RMC_SW_15, RMC_SW_3};
use telemetry::control::{ControlMessage, ControlSetting};
use telemetry::serial::core;
use telemetry::structures::{
    AlarmPriority, ControlAck, DataSnapshot, MachineStateSnapshot, TelemetryMessage,
};

use crate::config::environment::*;
use crate::utilities::types::DataPressure;
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChipState {
    Initializing,
    Running,
    Stopped,
    WaitingData,
    Error(String),
}

pub struct Chip {
    pub boot_time: Option<DateTime<Utc>>,
    pub last_tick: u64,
    pub data_pressure: DataPressure,
    pub last_machine_snapshot: MachineStateSnapshot,
    pub ongoing_alarms: HashMap<AlarmCode, AlarmPriority>,
    pub battery_level: Option<u8>,
    pub settings: ChipSettings,
    state: ChipState,
    lora_tx: Option<Sender<TelemetryMessage>>,
    channel_for_settings: Option<Sender<ControlMessage>>,
}

impl Chip {
    pub fn new(lora_sender: Option<Sender<TelemetryMessage>>) -> Chip {
        let last_machine_snapshot = MachineStateSnapshot::default();
        let cycles_per_minute = last_machine_snapshot.cpm_command;

        Chip {
            boot_time: None,
            last_tick: 0,
            data_pressure: VecDeque::with_capacity(GRAPH_NUMBER_OF_POINTS + 100),
            last_machine_snapshot,
            ongoing_alarms: HashMap::new(),
            battery_level: None,
            settings: ChipSettings::new(cycles_per_minute as usize),
            state: ChipState::WaitingData,
            lora_tx: lora_sender,
            channel_for_settings: None,
        }
    }

    pub fn new_event(&mut self, event: TelemetryMessage) {
        // Send to LORA?
        if let Some(lora_tx) = &self.lora_tx {
            if let Err(e) = lora_tx.send(event.clone()) {
                error!("an issue occured while sending data to lora: {:?}", e);
            }
        };

        // Handle actual event
        match event {
            TelemetryMessage::AlarmTrap(alarm) => {
                self.update_tick(alarm.systick);

                self.new_alarm(
                    alarm.alarm_code.into(),
                    alarm.alarm_priority,
                    alarm.triggered,
                );
            }

            TelemetryMessage::BootMessage(snapshot) => {
                self.reset(snapshot.systick);

                self.state = ChipState::Initializing;
            }

            TelemetryMessage::DataSnapshot(snapshot) => {
                self.clean_if_stopped();

                self.update_tick(snapshot.systick);
                self.add_pressure(&snapshot);

                self.battery_level = Some(snapshot.battery_level);
                self.state = ChipState::Running;
            }

            TelemetryMessage::MachineStateSnapshot(snapshot) => {
                self.clean_if_stopped();

                self.update_tick(snapshot.systick);
                self.update_cycles_per_minute(snapshot.cpm_command as usize);
                self.update_settings_values(&snapshot);

                for alarm in &snapshot.current_alarm_codes {
                    match AlarmPriority::try_from(*alarm) {
                        Ok(priority) => self.new_alarm((*alarm).into(), priority, true),
                        Err(e) => warn!(
                            "skip alarm {} because we couldn't get the priority: {:?}",
                            alarm, e
                        ),
                    };
                }

                self.last_machine_snapshot = snapshot;

                self.state = ChipState::Running;
            }

            TelemetryMessage::StoppedMessage(message) => {
                self.update_tick(message.systick);

                self.state = ChipState::Stopped;
            }

            TelemetryMessage::ControlAck(ack) => {
                self.update_on_ack(ack);
            }
        };
    }

    pub fn handle_settings_events(&mut self, events: Vec<ChipSettingsEvent>) {
        for event in events {
            let message = self.settings.new_settings_event(event);

            debug!(
                "handled setting event: {:?}, sender: {:?}",
                message, self.channel_for_settings
            );

            if let Some(tx) = &self.channel_for_settings {
                if let Err(err) = tx.send(message.clone()) {
                    error!(
                        "error sending message {:?} to the control unit: {:?}",
                        message, err
                    );
                } else {
                    debug!("setting message {:?} sent", message);
                }
            }
        }
    }

    pub fn new_error(&mut self, error: core::Error) {
        match error.kind() {
            core::ErrorKind::NoDevice => self.state = ChipState::WaitingData,
            err => self.state = ChipState::Error(format!("{:?}", err)),
        };
    }

    fn new_alarm(&mut self, code: AlarmCode, priority: AlarmPriority, triggered: bool) {
        if triggered {
            self.ongoing_alarms.insert(code, priority); // If we ever receive the same alarm, just replace the one we have
        } else {
            self.ongoing_alarms.remove(&code);
        }
    }

    fn add_pressure(&mut self, snapshot: &DataSnapshot) {
        assert!(self.boot_time.is_some());

        let snapshot_time =
            self.boot_time.unwrap() + Duration::microseconds(snapshot.systick as i64);

        // Fetch last pressure value in order to reduce noise
        let last_pressure = if let Some(last_pressure_inner) = self.data_pressure.get(0) {
            last_pressure_inner.1
        } else {
            0
        };

        // Low pass filter
        let new_point = last_pressure as i16
            - ((last_pressure as i16 - snapshot.pressure as i16)
                / TELEMETRY_POINTS_LOW_PASS_DEGREE as i16);

        // Points are stored as mmH20 (for more precision; though we do work in cmH20)
        self.data_pressure
            .push_front((snapshot_time, new_point as u16));
    }

    pub fn get_battery_level(&self) -> Option<u8> {
        self.battery_level
    }

    pub fn get_state(&self) -> &ChipState {
        &self.state
    }

    fn update_boot_time(&mut self) {
        let now = Utc::now();
        let duration = chrono::Duration::microseconds(self.last_tick as i64);

        self.boot_time = Some(now - duration);
    }

    fn update_tick(&mut self, tick: u64) {
        // Sometimes, a MachineStateSnapshot event can be received with an older systick
        // This is due to the way the systick is computed on the Makair's side. If we are too close of an ending millisecond
        // the micro() call will probably return the microseconds of the next millisecond, thus making a wrong order
        // If we have less than 1ms of difference between the messages, just ignore the systick update
        let to_update = if tick < self.last_tick {
            self.last_tick - tick > 1000
        } else {
            true
        };

        if to_update {
            if tick < self.last_tick {
                self.reset(tick);
            } else {
                self.last_tick = tick;

                if self.boot_time.is_none() {
                    self.update_boot_time();
                }
            }
        }
    }

    fn clean_if_stopped(&mut self) {
        if self.state == ChipState::Stopped {
            self.data_pressure.clear();
            self.last_machine_snapshot = MachineStateSnapshot::default();
        }
    }

    pub fn reset(&mut self, new_tick: u64) {
        self.last_tick = new_tick;
        self.data_pressure.clear();
        self.last_machine_snapshot = MachineStateSnapshot::default();
        self.ongoing_alarms.clear();

        self.update_boot_time();
    }

    pub fn clean_expired_pressure(&mut self) {
        if !self.data_pressure.is_empty() {
            let older = self.data_pressure.front().unwrap().0
                - chrono::Duration::seconds(GRAPH_DRAW_SECONDS as _);

            while self
                .data_pressure
                .back()
                .map(|p| p.0 < older)
                .unwrap_or(false)
            {
                self.data_pressure.pop_back();
            }
        }
    }

    fn deduplicate_alarms(alarms: &mut HashMap<AlarmCode, AlarmPriority>) {
        if alarms.contains_key(&AlarmCode::from(RMC_SW_11))
            && alarms.contains_key(&AlarmCode::from(RMC_SW_12))
        {
            alarms.remove(&AlarmCode::from(RMC_SW_11));
        }

        if alarms.contains_key(&AlarmCode::from(RMC_SW_1))
            && alarms.contains_key(&AlarmCode::from(RMC_SW_14))
        {
            alarms.remove(&AlarmCode::from(RMC_SW_14));
        }

        if alarms.contains_key(&AlarmCode::from(RMC_SW_3))
            && alarms.contains_key(&AlarmCode::from(RMC_SW_15))
        {
            alarms.remove(&AlarmCode::from(RMC_SW_15));
        }
    }

    pub fn ongoing_alarms_sorted(&self) -> Vec<(AlarmCode, AlarmPriority)> {
        let mut ongoing_alarms = self.ongoing_alarms.clone();

        Chip::deduplicate_alarms(&mut ongoing_alarms);

        let mut vec_alarms: Vec<(AlarmCode, AlarmPriority)> = ongoing_alarms
            .iter()
            .map(|(code, priority)| (*code, priority.clone()))
            .collect();

        vec_alarms.sort_by(|(code1, _), (code2, _)| code1.cmp(&code2));

        vec_alarms
    }

    pub fn init_settings_receiver(&mut self) -> Receiver<ControlMessage> {
        let channel = mpsc::channel();
        self.channel_for_settings = Some(channel.0);
        channel.1
    }

    fn update_cycles_per_minute(&mut self, cycles_per_minute: usize) {
        self.settings
            .expiration_term
            .set_cycles_per_minute(cycles_per_minute);
    }

    fn update_settings_values(&mut self, snapshot: &MachineStateSnapshot) {
        // Update expiratory term values
        self.settings.expiration_term.expiratory_term = snapshot.expiratory_term as usize;

        // Update trigger values
        self.settings.trigger.state = if snapshot.trigger_enabled {
            SettingsTriggerState::Enabled
        } else {
            SettingsTriggerState::Disabled
        };

        self.settings.trigger.inspiratory_trigger_offset = snapshot.trigger_offset as usize;
    }

    fn update_on_ack(&mut self, ack: ControlAck) {
        match ack.setting {
            ControlSetting::PeakPressure => {
                // TODO: update chip setting from there
                self.last_machine_snapshot.peak_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::PlateauPressure => {
                // TODO: update chip setting from there
                self.last_machine_snapshot.plateau_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::PEEP => {
                // TODO: update chip setting from there
                self.last_machine_snapshot.peep_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::CyclesPerMinute => {
                self.settings.cycles.cycles_per_minute = ack.value as usize;
                self.last_machine_snapshot.cpm_command = ack.value as u8
            }

            ControlSetting::TriggerEnabled => {
                if ack.value == 0 {
                    self.settings.trigger.state = SettingsTriggerState::Disabled;
                    self.last_machine_snapshot.trigger_enabled = false;
                } else {
                    self.settings.trigger.state = SettingsTriggerState::Enabled;
                    self.last_machine_snapshot.trigger_enabled = true;
                }
            }

            ControlSetting::TriggerOffset => {
                self.settings.trigger.inspiratory_trigger_offset = ack.value as usize;
                self.last_machine_snapshot.trigger_offset = ack.value as u8;
            }

            ControlSetting::ExpiratoryTerm => {
                self.settings.expiration_term.expiratory_term = ack.value as usize;
                self.last_machine_snapshot.expiratory_term = ack.value as u8;
            }
        }
    }
}
