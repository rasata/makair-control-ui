// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{
        self, canvas, primitive::shape::Style, rounded_rectangle::RoundedRectangle, Id as WidgetId,
    },
    Positionable, Sizeable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct ModalWidgetConfig {
    pub parent: WidgetId,
    pub background: WidgetId,
    pub container_borders: WidgetId,
    pub container: WidgetId,
    pub validate: Option<(WidgetId, WidgetId)>,
    pub width: f64,
    pub height: f64,
    pub padding: Option<f64>,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: ModalWidgetConfig) -> f64 {
    let mut style = canvas::Style::default();

    style.color = Some(Color::Rgba(0.0, 0.0, 0.0, 0.75));
    style.border = Some(0.0);
    style.border_color = Some(color::TRANSPARENT);

    canvas::Canvas::new()
        .with_style(style)
        .w_h(
            DISPLAY_WINDOW_SIZE_WIDTH as _,
            DISPLAY_WINDOW_SIZE_HEIGHT as _,
        )
        .x_y(0.0, 0.0)
        .set(config.background, &mut master.ui);

    let container_borders_style = Style::Fill(Some(Color::Rgba(
        81.0 / 255.0,
        81.0 / 255.0,
        81.0 / 255.0,
        1.0,
    )));
    RoundedRectangle::styled(
        [config.width + 5.0, config.height + 5.0],
        DISPLAY_ROUNDED_RECTANGLES_ROUND,
        container_borders_style,
    )
    .middle_of(config.parent)
    .set(config.container_borders, &mut master.ui);

    let mut container_style = canvas::Style::default();
    container_style.color = Some(Color::Rgba(26.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0, 1.0));
    container_style.border = Some(0.0);
    container_style.border_color = Some(color::TRANSPARENT);

    let mut container = canvas::Canvas::new()
        .with_style(container_style)
        .w_h(config.width, config.height)
        .middle_of(config.container_borders);

    if let Some(padding) = config.padding {
        container = container.pad(padding);
    }

    container.set(config.container, &mut master.ui);

    if let Some((validate_button, validate_text)) = config.validate {
        let button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

        widget::RoundedRectangle::styled(
            [MODAL_VALIDATE_BUTTON_WIDTH, MODAL_VALIDATE_BUTTON_HEIGHT],
            15.0,
            button_style,
        )
        .bottom_right_of(config.container)
        .set(validate_button, &mut master.ui);

        let mut validate_text_style = widget::text::Style::default();
        validate_text_style.font_id = Some(Some(master.fonts.bold));
        validate_text_style.color = Some(color::BLACK);
        validate_text_style.font_size = Some(16);

        widget::Text::new(&APP_I18N.t("modal-close"))
            .with_style(validate_text_style)
            .mid_top_with_margin_on(validate_button, 4.0)
            .set(validate_text, &mut master.ui);
    }

    0 as _
}