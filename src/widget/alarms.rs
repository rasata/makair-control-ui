// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::cmp::max;

use conrod_core::widget::id::List;
use conrod_core::widget::Id as WidgetId;
use conrod_core::{
    color::{self, Color},
    widget::{self, rounded_rectangle::RoundedRectangle},
    Colorable, Positionable, Sizeable, Widget,
};

use makair_telemetry::alarm::AlarmCode;
use makair_telemetry::structures::AlarmPriority;

use super::alarm;

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

const CONTAINER_WITH_ALARMS_BACKGROUND_COLOR: Color =
    Color::Rgba(42.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 0.96);
const CONTAINER_WITHOUT_ALARMS_BACKGROUND_COLOR: Color =
    Color::Rgba(17.0 / 255.0, 17.0 / 255.0, 17.0 / 255.0, 0.96);

pub struct Config<'a> {
    pub parent: WidgetId,
    pub container: WidgetId,

    pub title_wrapper: WidgetId,
    pub title: WidgetId,

    pub empty: WidgetId,

    pub alarm_widgets: &'a List,
    pub alarm_codes_containers: &'a List,
    pub alarm_codes: &'a List,
    pub alarm_messages_containers: &'a List,
    pub alarm_messages: &'a List,

    pub alarms: &'a [(AlarmCode, AlarmPriority)],
}

pub fn render(master: &mut ControlWidget, config: Config) -> f64 {
    // Count alarm totals
    let alarms_count = config.alarms.len();

    // Compute container width based on alarm count (the container should be small if there are \
    //   no alarms, wide if there are alarms)
    let container_width = if alarms_count == 0 {
        DISPLAY_ALARM_CONTAINER_WIDTH_BASE
    } else {
        DISPLAY_ALARM_CONTAINER_WIDTH_BASE
            + (BRANDING_WIDTH as f64)
            + DISPLAY_ALARM_CONTAINER_WIDTH_HAS_ALARMS_OFFSET
    };

    // Compute container left margin (based on alarms count)
    let container_margin_left = if alarms_count == 0 {
        DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_BASE
    } else {
        DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_HAS_ALARMS
    };

    // Compute container height (based on alarms count)
    let mut container_height = (max(1, alarms_count) as f64) * DISPLAY_ALARM_MESSAGE_HEIGHT
        + DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL
        + DISPLAY_ALARM_MESSAGE_SPACING_BOTTOM_INITIAL;

    if alarms_count > 1 {
        container_height += (alarms_count as f64) * DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER;
    }

    // Acquire container background color (based on alarms count)
    let container_background_color = if alarms_count > 0 {
        CONTAINER_WITH_ALARMS_BACKGROUND_COLOR
    } else {
        CONTAINER_WITHOUT_ALARMS_BACKGROUND_COLOR
    };

    // Draw container box
    RoundedRectangle::fill_with(
        [container_width, container_height],
        DISPLAY_ROUNDED_RECTANGLES_ROUND,
        container_background_color,
    )
    .right_from(config.parent, container_margin_left)
    .down_from(config.parent, -1.0 * BRANDING_HEIGHT as f64)
    .set(config.container, &mut master.ui);

    // Create title wrapper canvas
    gen_widget_container!(
        master,
        container_id: config.title_wrapper,
        color: color::TRANSPARENT,
        width: DISPLAY_ALARM_TITLE_WRAPPER_WIDTH,
        height: DISPLAY_ALARM_TITLE_WRAPPER_HEIGHT,
        positions: top_left_with_margins_on[
            config.container,
            DISPLAY_ALARM_CONTAINER_PADDING_TOP,
            if alarms_count > 0 {
                DISPLAY_ALARM_CONTAINER_HAS_ALARMS_PADDING_LEFT
            } else {
                DISPLAY_ALARM_CONTAINER_EMPTY_PADDING_LEFT
            },
        ]
    );

    // Initialize title text style
    // Notice: the first text layer needs to be positionned using relative coordinates, and \
    //   cannot be positionned using a 'mid' auto coordinate, as this has been seen to center \
    //   vertically with a slight offset, which would make the text look uncentered to the \
    //   human eye.
    let mut title_text_style = conrod_core::widget::primitive::text::Style::default();

    title_text_style.font_id = Some(Some(master.fonts.bold));
    title_text_style.color = Some(color::WHITE);
    title_text_style.font_size = Some(14);

    // Insert text in canvas
    widget::text::Text::new(&APP_I18N.t("alarms-title"))
        .with_style(title_text_style)
        .top_left_of(config.title_wrapper)
        .set(config.title, &mut master.ui);

    // Append all alarms?
    // Notice: only draw alarms box if there are active alarms
    if alarms_count > 0 {
        for x in 0..alarms_count {
            let (code, alarm) = config.alarms.get(x).unwrap();

            alarm::render(master, &config, *code, alarm, x);
        }
    } else {
        widget::text::Text::new(&APP_I18N.t("alarms-empty"))
            .color(Color::Rgba(1.0, 1.0, 1.0, 0.5))
            .font_size(12)
            .right_from(
                config.title_wrapper,
                DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_EMPTY_SPACING,
            )
            .y_relative(0.0)
            .set(config.empty, &mut master.ui);
    }

    0 as _
}
