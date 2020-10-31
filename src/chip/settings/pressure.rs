// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingAction;

const PRESSURE_STEP: usize = 10;

#[derive(Debug)]
pub enum SettingsPressureEvent {
    Peak(SettingAction),
    Plateau(SettingAction),
    PEEP(SettingAction),
}

#[derive(Debug)]
pub struct SettingsPressure {
    pub peak: usize,
    pub plateau: usize,
    pub peep: usize,
}

impl SettingsPressure {
    pub fn new() -> SettingsPressure {
        SettingsPressure {
            peak: ControlSetting::PeakPressure.default(),
            plateau: ControlSetting::PlateauPressure.default(),
            peep: ControlSetting::PEEP.default(),
        }
    }

    pub fn new_event(&self, event: SettingsPressureEvent) -> ControlMessage {
        match event {
            SettingsPressureEvent::Peak(action) => self.set_peak(action),
            SettingsPressureEvent::Plateau(action) => self.set_plateau(action),
            SettingsPressureEvent::PEEP(action) => self.set_peep(action),
        }
    }

    fn set_peak(&self, action: SettingAction) -> ControlMessage {
        let control = ControlSetting::PeakPressure;

        ControlMessage {
            setting: control,
            value: self.acquire_new_value(&control, action, self.peak) as u16,
        }
    }

    fn set_plateau(&self, action: SettingAction) -> ControlMessage {
        let control = ControlSetting::PlateauPressure;

        ControlMessage {
            setting: control,
            value: self.acquire_new_value(&control, action, self.plateau) as u16,
        }
    }

    fn set_peep(&self, action: SettingAction) -> ControlMessage {
        let control = ControlSetting::PEEP;

        ControlMessage {
            setting: control,
            value: self.acquire_new_value(&control, action, self.peep) as u16,
        }
    }

    fn acquire_new_value(
        &self,
        control: &ControlSetting,
        action: SettingAction,
        previous_value: usize,
    ) -> usize {
        match action {
            SettingAction::More => {
                let new_value = previous_value + PRESSURE_STEP;

                if control.bounds().contains(&new_value) {
                    new_value
                } else {
                    previous_value
                }
            }

            SettingAction::Less => {
                if previous_value >= PRESSURE_STEP {
                    let new_value = previous_value - PRESSURE_STEP;

                    if control.bounds().contains(&new_value) {
                        new_value
                    } else {
                        previous_value
                    }
                } else {
                    previous_value
                }
            }
        }
    }
}