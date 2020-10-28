// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;

pub struct LayoutWidgetSliceConfig {
    parent: WidgetId,
    top: f64,
    height: f64,
    layout: WidgetId,
}

pub struct LayoutWidgetConfig {
    header: LayoutWidgetSliceConfig,
    body: LayoutWidgetSliceConfig,
    footer: LayoutWidgetSliceConfig,
}

impl LayoutWidgetSliceConfig {
    pub fn new(
        parent: WidgetId,
        top: f64,
        height: f64,
        layout: WidgetId,
    ) -> LayoutWidgetSliceConfig {
        LayoutWidgetSliceConfig {
            parent,
            top,
            height,
            layout,
        }
    }
}

impl LayoutWidgetConfig {
    pub fn new(
        header: LayoutWidgetSliceConfig,
        body: LayoutWidgetSliceConfig,
        footer: LayoutWidgetSliceConfig,
    ) -> LayoutWidgetConfig {
        LayoutWidgetConfig {
            header,
            body,
            footer,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: LayoutWidgetConfig) -> f64 {
    // Create body layout rectangle
    widget::Rectangle::fill_with(
        [DISPLAY_WINDOW_SIZE_WIDTH as _, config.body.height],
        color::TRANSPARENT,
    )
    .top_left_with_margins_on(config.body.parent, config.body.top, 0.0)
    .set(config.body.layout, &mut master.ui);

    // Create footer layout rectangle
    widget::Rectangle::fill_with(
        [DISPLAY_WINDOW_SIZE_WIDTH as _, config.footer.height],
        color::TRANSPARENT,
    )
    .down_from(config.footer.parent, config.footer.top)
    .set(config.footer.layout, &mut master.ui);

    // Create header layout rectangle
    // Notice: this block is defined after the others because we want it to overflow and be on top \
    //   of the screen.
    widget::Rectangle::fill_with(
        [DISPLAY_WINDOW_SIZE_WIDTH as _, config.header.height],
        color::TRANSPARENT,
    )
    .top_left_of(config.header.parent)
    .set(config.header.layout, &mut master.ui);

    0.0
}
