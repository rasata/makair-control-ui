// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub const RUNTIME_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const WINDOW_ICON_WIDTH: u32 = 512;
pub const WINDOW_ICON_HEIGHT: u32 = 512;

pub const DISPLAY_WINDOW_SIZE_WIDTH: u32 = 800;
pub const DISPLAY_WINDOW_SIZE_HEIGHT: u32 = 480;
pub const DISPLAY_FRAMERATE_SMOOTH_HEAVY: u64 = 30;
pub const DISPLAY_FRAMERATE_MODERATE_FAST: u64 = 20;
pub const DISPLAY_FRAMERATE_JERKY_FAST: u64 = 8;

pub const BOOTLOADER_LOGO_WIDTH: u32 = 98;
pub const BOOTLOADER_LOGO_HEIGHT: u32 = 96;

pub const ERROR_WIDTH: f64 = 180.0;
pub const ERROR_HEIGHT: f64 = 186.0;
pub const ERROR_ICON_WIDTH: u32 = 115;
pub const ERROR_ICON_HEIGHT: u32 = 101;
pub const ERROR_TEXT_BOX_HEIGHT: f64 = 50.0;
pub const ERROR_TITLE_FONT_SIZE: u32 = 19;
pub const ERROR_MESSAGE_FONT_SIZE: u32 = 16;

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

pub const CONTROLS_WRAPPER_HEIGHT: f64 = 44.0;
pub const CONTROLS_WRAPPER_WIDTH: f64 =
    (CONTROLS_BUTTON_FOOTPRINT_WIDTH * CONTROLS_BUTTONS_COUNT) - CONTROLS_BUTTON_MARGIN_LEFT;
pub const CONTROLS_WRAPPER_MARGIN_TOP: f64 = 4.0;
pub const CONTROLS_WRAPPER_MARGIN_RIGHT: f64 = 194.0;
pub const CONTROLS_BUTTONS_COUNT: f64 = 3.0;
pub const CONTROLS_BUTTON_DIAMETER: f64 = CONTROLS_WRAPPER_HEIGHT;
pub const CONTROLS_BUTTON_RADIUS: f64 = CONTROLS_BUTTON_DIAMETER / 2.0;
pub const CONTROLS_BUTTON_FOOTPRINT_WIDTH: f64 =
    CONTROLS_BUTTON_DIAMETER + CONTROLS_BUTTON_MARGIN_LEFT;
pub const CONTROLS_BUTTON_MARGIN_LEFT: f64 = 4.0;

pub const STATUS_WRAPPER_WIDTH: f64 = 112.0;
pub const STATUS_WRAPPER_HEIGHT: f64 = 42.0;
pub const STATUS_WRAPPER_MARGIN_TOP: f64 = 5.0;
pub const STATUS_WRAPPER_MARGIN_RIGHT: f64 = 66.0;
pub const STATUS_BOX_TEXT_MARGIN_TOP: f64 = 3.0;
pub const STATUS_BOX_RECORDING_PADDING_LEFT: f64 = 12.0;
pub const STATUS_BOX_RECORDING_PADDING_RIGHT: f64 = 5.0;
pub const STATUS_RECORDING_OUTER_RADIUS: f64 = 6.0;
pub const STATUS_RECORDING_INNER_RADIUS: f64 = 5.0;

pub const HEARTBEAT_GROUND_DIAMETER: f64 = 14.0;
pub const HEARTBEAT_SURROUND_MARGIN_TOP: f64 = 4.0;
pub const HEARTBEAT_SURROUND_MARGIN_RIGHT: f64 = 6.0;
pub const HEARTBEAT_SURROUND_DIAMETER: f64 = 44.0;
pub const HEARTBEAT_SURROUND_THICKNESS: f64 = 2.0;
pub const HEARTBEAT_INNER_MAX_OVERFLOW: u16 = 4;

pub const DISPLAY_ALARM_CONTAINER_WIDTH_BASE: f64 = 305.0;
pub const DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_BASE: f64 = 20.0;
pub const DISPLAY_ALARM_CONTAINER_WIDTH_HAS_ALARMS_OFFSET: f64 =
    DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_BASE;
pub const DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_HAS_ALARMS: f64 = -1.0 * BRANDING_WIDTH as f64;
pub const DISPLAY_ALARM_CONTAINER_PADDING_TOP: f64 = 12.5;
pub const DISPLAY_ALARM_CONTAINER_EMPTY_PADDING_LEFT: f64 = 22.0;
pub const DISPLAY_ALARM_CONTAINER_HAS_ALARMS_PADDING_LEFT: f64 = 16.0;
pub const DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_EMPTY_SPACING: f64 = 38.0;
pub const DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_HAS_ALARMS_SPACING: f64 = 20.0;

pub const DISPLAY_ALARM_TITLE_WRAPPER_WIDTH: f64 = 65.0;
pub const DISPLAY_ALARM_TITLE_WRAPPER_HEIGHT: f64 = 18.0;

pub const DISPLAY_ALARM_MESSAGE_WIDTH: f64 = 302.0;
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

pub const TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE: i32 = 10;
pub const TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE: i32 = 100;
pub const TELEMETRY_POINTS_PER_SECOND: usize = 40;
pub const TELEMETRY_POINTS_LOW_PASS_DEGREE: i16 = 2;

pub const TELEMETRY_WIDGET_BOTTOM_COUNT: f64 = 4.0;
pub const TELEMETRY_WIDGET_RIGHT_COUNT: f64 = 3.0;
pub const TELEMETRY_WIDGET_SPACING_SIDES: f64 = 4.0;
pub const TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH: f64 = DISPLAY_WINDOW_SIZE_WIDTH as f64
    / TELEMETRY_WIDGET_BOTTOM_COUNT
    - TELEMETRY_WIDGET_SPACING_SIDES * (TELEMETRY_WIDGET_BOTTOM_COUNT - 1.0)
        / TELEMETRY_WIDGET_BOTTOM_COUNT;
pub const TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT: f64 = 39.0;
pub const TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT_WITH_SPACING: f64 =
    TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT + TELEMETRY_WIDGET_SPACING_SIDES;
pub const TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE: u32 = 14;
pub const TELEMETRY_WIDGET_RIGHT_MODE_SEPARATOR_SPACING: f64 = 9.0;
pub const TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH: f64 = (DISPLAY_WINDOW_SIZE_WIDTH - GRAPH_WIDTH) as f64;
pub const TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT: f64 = (GRAPH_HEIGHT as f64
    - TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT_WITH_SPACING)
    / TELEMETRY_WIDGET_RIGHT_COUNT
    - TELEMETRY_WIDGET_SPACING_SIDES;
pub const TELEMETRY_WIDGET_RIGHT_POSITION_Y_BASE: f64 = GRAPH_HEIGHT as f64
    + LAYOUT_FOOTER_SIZE_HEIGHT
    - TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT_WITH_SPACING;
pub const TELEMETRY_WIDGET_PADDING_LEFT: f64 = 7.0;
pub const TELEMETRY_WIDGET_UNIT_PADDING_BOTTOM_TOP: f64 = 8.0;
pub const TELEMETRY_WIDGET_UNIT_BORDER_RADIUS: f64 = 5.0;
pub const TELEMETRY_WIDGET_TITLE_FONT_SIZE: u32 = 16;
pub const TELEMETRY_WIDGET_UNIT_FONT_SIZE: u32 = 12;
pub const TELEMETRY_WIDGET_VALUE_EMPTY: &str = "--";

pub const TELEMETRY_ARROW_MAIN_WIDTH: u32 = 10;
pub const TELEMETRY_ARROW_MAIN_HEIGHT: u32 = 11;
pub const TELEMETRY_ARROW_LINE_WIDTH: u32 = 6;
pub const TELEMETRY_ARROW_LINE_HEIGHT: u32 = 3;
pub const TELEMETRY_ARROW_SPACING_SIDES: f64 = 5.0;

pub const GRAPH_DRAW_SECONDS: i64 = 5;
pub const GRAPH_DRAW_PRESSURE_RANGE_LOW: i32 = -10;
pub const GRAPH_DRAW_PRESSURE_RANGE_LOW_PRECISION_DIVIDED: i32 =
    GRAPH_DRAW_PRESSURE_RANGE_LOW * TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE;
pub const GRAPH_DRAW_PRESSURE_RANGE_HIGH: u8 = 45;
pub const GRAPH_DRAW_PRESSURE_RANGE_HIGH_PRECISION_DIVIDED: i32 =
    (GRAPH_DRAW_PRESSURE_RANGE_HIGH as i32) * TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE;
pub const GRAPH_DRAW_FLOW_RANGE_LOW: i32 = -GRAPH_DRAW_FLOW_RANGE_HIGH;
pub const GRAPH_DRAW_FLOW_RANGE_LOW_PRECISION_DIVIDED: i32 =
    GRAPH_DRAW_FLOW_RANGE_LOW * TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE;
pub const GRAPH_DRAW_FLOW_RANGE_HIGH: i32 = 70;
pub const GRAPH_DRAW_FLOW_RANGE_HIGH_PRECISION_DIVIDED: i32 =
    GRAPH_DRAW_FLOW_RANGE_HIGH * TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE;
pub const GRAPH_DRAW_MARGIN_TOP: u32 = 0;
pub const GRAPH_DRAW_MARGIN_BOTTOM: u32 = 10;
pub const GRAPH_DRAW_MARGIN_LEFT: u32 = 0;
pub const GRAPH_DRAW_MARGIN_RIGHT: u32 = 0;
pub const GRAPH_DRAW_LINE_SIZE: u32 = 2;
pub const GRAPH_DRAW_AXIS_SIZE: u32 = 1;
pub const GRAPH_DRAW_AXIS_FONT_SIZE: u32 = 14;
pub const GRAPH_DRAW_LABEL_WIDTH: u32 = 56;
pub const GRAPH_DRAW_LABEL_NUMBER_MAX: usize = 5;
pub const GRAPH_LABEL_BOX_WIDTH: f64 = (GRAPH_DRAW_LABEL_WIDTH - 2 * GRAPH_DRAW_AXIS_SIZE) as f64;
pub const GRAPH_LABEL_BOX_HEIGHT: f64 = 36.0;
pub const GRAPH_LABEL_BOX_FONT_SIZE: u32 = 11;
pub const GRAPH_SATURATE_LINE_THICKNESS: f64 = 3.0;
pub const GRAPH_NUMBER_OF_POINTS: usize = GRAPH_DRAW_SECONDS as usize * TELEMETRY_POINTS_PER_SECOND;
pub const GRAPH_WIDTH: u32 = 650;
pub const GRAPH_HEIGHT: u32 = 325;
pub const GRAPH_SPACING: f64 = 6.0;

pub const MAX_ALLOWED_PRESSURE_INITIAL_MINIMUM: f64 = 0.0;
pub const MAX_ALLOWED_PRESSURE_ALERT_ERROR_RATIO: f64 = 0.15;

pub const RUN_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const RUN_SETTINGS_MODAL_WIDTH: f64 = 600.0;
pub const RUN_SETTINGS_MODAL_HEIGTH: f64 = 140.0;
pub const RUN_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 240.0;

pub const SNOOZE_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const SNOOZE_SETTINGS_MODAL_WIDTH: f64 = 600.0;
pub const SNOOZE_SETTINGS_MODAL_HEIGTH: f64 = 140.0;
pub const SNOOZE_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 200.0;

pub const ADVANCED_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const ADVANCED_SETTINGS_MODAL_WIDTH: f64 = 600.0;
pub const ADVANCED_SETTINGS_MODAL_HEIGTH: f64 = 102.0
    + (ADVANCED_SETTINGS_LINES_COUNT as f64
        * (ADVANCED_SETTINGS_LINE_FONT_SIZE as f64 + ADVANCED_SETTINGS_LINE_MARGIN_TOP)
        - ADVANCED_SETTINGS_LINE_MARGIN_TOP);
pub const ADVANCED_SETTINGS_LINES_COUNT: usize = 14;
pub const ADVANCED_SETTINGS_LINE_MARGIN_TOP: f64 = 8.0;
pub const ADVANCED_SETTINGS_LINE_FONT_SIZE: u32 = 14;
pub const ADVANCED_SETTINGS_LINE_VALUE_PADDING_LEFT: f64 = 280.0;
pub const ADVANCED_SETTINGS_LINE_VALUE_EMPTY: &str = "--";

pub const MODE_SETTINGS_MODAL_PADDING: f64 = 20.0;
pub const MODE_SETTINGS_MODAL_WIDTH: f64 = 740.0;
pub const MODE_SETTINGS_MODAL_HEIGTH: f64 = 470.0;
pub const MODE_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 260.0;
pub const MODE_SETTINGS_MODAL_FORM_FIELD_HEIGHT_PADDED: f64 = 42.0;
pub const MODE_SETTINGS_SELECTOR_TABS_COUNT: usize = 5;
pub const MODE_SETTINGS_SELECTOR_TABS_HEIGHT: f64 = 48.0;
pub const MODE_SETTINGS_GROUP_TABS_COUNT: usize = 2;
pub const MODE_SETTINGS_GROUP_TABS_WIDTH: f64 = 130.0;
pub const MODE_SETTINGS_GROUP_TABS_HEIGHT: f64 = 40.0;
pub const MODE_SETTINGS_GROUP_TABS_MARGIN_RIGHT: f64 = 38.0;
pub const MODE_SETTINGS_GROUP_TABS_MARGIN_TOP: f64 = 14.0;
pub const MODE_SETTINGS_GROUP_TABS_BORDER_RADIUS: f64 = 3.0;

pub const BUTTON_HEIGHT: f64 = 30.0;
pub const BUTTON_BORDER_RADIUS: f64 = 15.0;

pub const MODAL_TEXT_FONT_SIZE: u32 = 18;
pub const MODAL_BUTTON_NAVIGATE_FONT_SIZE: u32 = 20;
pub const MODAL_BUTTON_NAVIGATE_VALUE_WIDTH: f64 = 120.0;
pub const MODAL_BUTTON_NAVIGATE_VALUE_HEIGHT: f64 = BUTTON_HEIGHT;
pub const MODAL_BUTTON_NAVIGATE_VALUE_MARGIN_TOP: f64 = 2.0;
pub const MODAL_BUTTON_NAVIGATE_VALUE_DECREASE: &str = "<";
pub const MODAL_BUTTON_NAVIGATE_VALUE_INCREASE: &str = ">";
pub const MODAL_BUTTON_NAVIGATE_WIDTH: f64 = 50.0;
pub const MODAL_BUTTON_NAVIGATE_PADDING_INNER: f64 = 20.0;
pub const MODAL_VALIDATE_BUTTON_WIDTH: f64 = 100.0;
pub const MODAL_VALIDATE_BUTTON_HEIGHT: f64 = 30.0;

pub const LAYOUT_HEADER_SIZE_HEIGHT: f64 = 65.0;
pub const LAYOUT_BODY_SIZE_HEIGHT: f64 = GRAPH_HEIGHT as _;
pub const LAYOUT_FOOTER_SIZE_HEIGHT: f64 = 90.0;

pub const LAYOUT_TEXTURE_HEADER_WIDTH: u32 = 606;
pub const LAYOUT_TEXTURE_HEADER_HEIGHT: u32 = 52;

#[cfg(feature = "lora")]
pub const LORA_GPIO_PIN_NUMBER: u64 = 25;
