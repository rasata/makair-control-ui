// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub const RUNTIME_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const WINDOW_ICON_WIDTH: u32 = 512;
pub const WINDOW_ICON_HEIGHT: u32 = 512;

pub const DISPLAY_WINDOW_SIZE_WIDTH: u32 = 800;
pub const DISPLAY_WINDOW_SIZE_HEIGHT: u32 = 480;
pub const DISPLAY_FRAMERATE_RUNNING: u64 = 24;
pub const DISPLAY_FRAMERATE_STOPPED: u64 = 8;

pub const BOOTLOADER_LOGO_WIDTH: u32 = 98;
pub const BOOTLOADER_LOGO_HEIGHT: u32 = 96;

pub const ERROR_PADDING_TOP: f64 = 16.0;
pub const ERROR_PADDING_LEFT: f64 = 22.0;
pub const ERROR_MESSAGE_FONT_SIZE: u32 = 18;

pub const INITIALIZING_WIDTH: f64 = 160.0;
pub const INITIALIZING_HEIGHT: f64 = 138.0;
pub const INITIALIZING_MESSAGE_FONT_SIZE: u32 = 20;

pub const BRANDING_WIDTH: u32 = 121;
pub const BRANDING_HEIGHT: u32 = 45;
pub const BRANDING_IMAGE_MARGIN_TOP: f64 = 3.0;
pub const BRANDING_IMAGE_MARGIN_LEFT: f64 = 5.0;
pub const BRANDING_TEXT_MARGIN_TOP: f64 = 33.0;
pub const BRANDING_TEXT_MARGIN_LEFT: f64 = 61.0;
pub const BRANDING_TEXT_VERSION_NONE: &str = "v0.0.0";

pub const STATUS_WRAPPER_WIDTH: f64 = 112.0;
pub const STATUS_WRAPPER_HEIGHT: f64 = 42.0;
pub const STATUS_WRAPPER_MARGIN_TOP: f64 = 5.0;
pub const STATUS_WRAPPER_MARGIN_RIGHT: f64 = 74.0;
pub const STATUS_BOX_TEXT_MARGIN_TOP: f64 = 3.0;
pub const STATUS_BOX_RECORDING_PADDING_LEFT: f64 = 12.0;
pub const STATUS_BOX_RECORDING_PADDING_RIGHT: f64 = 5.0;
pub const STATUS_SAVE_ICON_WIDTH: u32 = 24;
pub const STATUS_SAVE_ICON_HEIGHT: u32 = STATUS_SAVE_ICON_WIDTH;

pub const HEARTBEAT_GROUND_DIAMETER: f64 = 14.0;
pub const HEARTBEAT_SURROUND_MARGIN_TOP: f64 = 4.0;
pub const HEARTBEAT_SURROUND_MARGIN_RIGHT: f64 = 6.0;
pub const HEARTBEAT_SURROUND_DIAMETER: f64 = 44.0;
pub const HEARTBEAT_SURROUND_THICKNESS: f64 = 2.0;
pub const HEARTBEAT_INNER_MAX_OVERFLOW: u16 = 4;

pub const DISPLAY_ALARM_CONTAINER_WIDTH_BASE: f64 = 426.0;
pub const DISPLAY_ALARM_CONTAINER_WIDTH_HAS_ALARMS_OFFSET: f64 = 44.0;
pub const DISPLAY_ALARM_CONTAINER_PADDING_TOP: f64 = 12.5;
pub const DISPLAY_ALARM_CONTAINER_EMPTY_PADDING_LEFT: f64 = 22.0;
pub const DISPLAY_ALARM_CONTAINER_HAS_ALARMS_PADDING_LEFT: f64 = 16.0;
pub const DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_EMPTY_SPACING: f64 = 38.0;
pub const DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_HAS_ALARMS_SPACING: f64 = 20.0;

pub const DISPLAY_ALARM_TITLE_WRAPPER_WIDTH: f64 = 65.0;
pub const DISPLAY_ALARM_TITLE_WRAPPER_HEIGHT: f64 = 18.0;

pub const DISPLAY_ALARM_MESSAGE_WIDTH: f64 = 438.0;
pub const DISPLAY_ALARM_MESSAGE_HEIGHT: f64 = 30.0;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER: f64 = 4.0;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL_OFFSET: f64 = 5.0;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL: f64 = 10.0;
pub const DISPLAY_ALARM_MESSAGE_SPACING_BOTTOM_INITIAL: f64 = 5.0;
pub const DISPLAY_ALARM_MESSAGE_FONT_SIZE: u32 = 17;

pub const DISPLAY_ALARM_CODE_WIDTH: f64 = 32.0;
pub const DISPLAY_ALARM_CODE_HEIGHT: f64 = DISPLAY_ALARM_MESSAGE_HEIGHT;
pub const DISPLAY_ALARM_CODE_FONT_SIZE: u32 = 19;

pub const DISPLAY_ROUNDED_RECTANGLES_ROUND: f64 = 2.0;

pub const DISPLAY_STOP_MESSAGE_CONTAINER_WIDTH: f64 = 380.0;
pub const DISPLAY_STOP_MESSAGE_CONTAINER_HEIGHT: f64 = 84.0;
pub const DISPLAY_STOP_MESSAGE_PADDING_TOP: f64 = 16.0;
pub const DISPLAY_STOP_MESSAGE_PADDING_BOTTOM: f64 = 22.0;

pub const TELEMETRY_POINTS_PRECISION_DIVIDE: u16 = 10;
pub const TELEMETRY_POINTS_PER_SECOND: usize = 100;
pub const TELEMETRY_POINTS_LOW_PASS_DEGREE: u16 = 4;

pub const TELEMETRY_WIDGET_SIZE_WIDTH: f64 = DISPLAY_WINDOW_SIZE_WIDTH as f64 / 4.0;
pub const TELEMETRY_WIDGET_PADDING_LEFT: f64 = 7.0;
pub const TELEMETRY_WIDGET_PADDING_LEFT_DEEP: f64 = 10.0;
pub const TELEMETRY_WIDGET_UNIT_PADDING_BOTTOM_TOP: f64 = 8.0;
pub const TELEMETRY_WIDGET_TITLE_FONT_SIZE: u32 = 16;
pub const TELEMETRY_WIDGET_UNIT_FONT_SIZE: u32 = 12;
pub const TELEMETRY_WIDGET_LABELS_FONT_SIZE: u32 = 13;
pub const TELEMETRY_WIDGET_LABELS_LABEL_WIDTH: f64 = 58.0;
pub const TELEMETRY_WIDGET_VALUE_EMPTY: &str = "--";

pub const TELEMETRY_ARROW_WIDTH: u32 = 15;
pub const TELEMETRY_ARROW_HEIGHT: u32 = 9;
pub const TELEMETRY_ARROW_SPACING_SIDES: f64 = 5.0;

pub const GRAPH_DRAW_SECONDS: usize = 5;
pub const GRAPH_DRAW_RANGE_LOW: i32 = 0;
pub const GRAPH_DRAW_MARGIN_TOP: u32 = 0;
pub const GRAPH_DRAW_MARGIN_BOTTOM: u32 = 10;
pub const GRAPH_DRAW_MARGIN_LEFT: u32 = 0;
pub const GRAPH_DRAW_MARGIN_RIGHT: u32 = 0;
pub const GRAPH_DRAW_LINE_SIZE: u32 = 2;
pub const GRAPH_DRAW_POINT_SIZE: u32 = 0;
pub const GRAPH_DRAW_LABEL_WIDTH: u32 = 28;
pub const GRAPH_DRAW_LABEL_NUMBER_MAX: usize = 5;
pub const GRAPH_NUMBER_OF_POINTS: usize = GRAPH_DRAW_SECONDS * TELEMETRY_POINTS_PER_SECOND;
pub const GRAPH_WIDTH: u32 = 650;
pub const GRAPH_HEIGHT: u32 = 315;

pub const PEAK_PRESSURE_INITIAL_MIN: f64 = 0.0;
pub const PEAK_PRESSURE_ALERT_ERROR_RATIO: f64 = 0.15;
pub const CYCLE_RATIO_INSPIRATION: u8 = 1;

pub const TRIGGER_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const TRIGGER_SETTINGS_MODAL_WIDTH: f64 = 600.0;
pub const TRIGGER_SETTINGS_MODAL_HEIGTH: f64 = 220.0;
pub const TRIGGER_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 300.0;

pub const EXPIRATION_TERM_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const EXPIRATION_TERM_SETTINGS_MODAL_WIDTH: f64 = 520.0;
pub const EXPIRATION_TERM_SETTINGS_MODAL_HEIGTH: f64 = 140.0;
pub const EXPIRATION_TERM_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 220.0;

pub const PRESSURE_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const PRESSURE_SETTINGS_MODAL_WIDTH: f64 = 560.0;
pub const PRESSURE_SETTINGS_MODAL_HEIGTH: f64 = 280.0;
pub const PRESSURE_SETTINGS_MODAL_FORM_ROW_MARGIN_TOP: f64 = 50.0;
pub const PRESSURE_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 240.0;

pub const CYCLES_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const CYCLES_SETTINGS_MODAL_WIDTH: f64 = 520.0;
pub const CYCLES_SETTINGS_MODAL_HEIGTH: f64 = 140.0;
pub const CYCLES_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 220.0;

pub const BUTTON_HEIGHT: f64 = 30.0;
pub const BUTTON_BORDER_RADIUS: f64 = 15.0;

pub const MODAL_TEXT_FONT_SIZE: u32 = 18;
pub const MODAL_BUTTON_NAVIGATE_FONT_SIZE: u32 = 20;
pub const MODAL_BUTTON_NAVIGATE_VALUE_MARGIN_TOP: f64 = 2.0;
pub const MODAL_BUTTON_NAVIGATE_VALUE_DECREASE: &str = "<";
pub const MODAL_BUTTON_NAVIGATE_VALUE_INCREASE: &str = ">";
pub const MODAL_BUTTON_NAVIGATE_WIDTH: f64 = 50.0;
pub const MODAL_BUTTON_NAVIGATE_PADDING_INNER: f64 = 20.0;
pub const MODAL_VALIDATE_BUTTON_WIDTH: f64 = 100.0;
pub const MODAL_VALIDATE_BUTTON_HEIGHT: f64 = 30.0;

pub const LAYOUT_HEADER_SIZE_HEIGHT: f64 = 65.0;
pub const LAYOUT_BODY_SIZE_HEIGHT: f64 = GRAPH_HEIGHT as _;
pub const LAYOUT_FOOTER_SIZE_HEIGHT: f64 = 100.0;

#[cfg(feature = "lora")]
pub const LORA_GPIO_PIN_NUMBER: u64 = 25;

#[cfg(not(feature = "graph-scaler"))]
pub const GRAPH_DRAW_RANGE_HIGH_STATIC_INITIAL: u8 = 65;

#[cfg(feature = "graph-scaler")]
pub const GRAPH_DRAW_RANGE_HIGH_DYNAMIC_INITIAL: u8 = 20;
