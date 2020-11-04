// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_widget_container {
    (
        $master:ident,
        container_id: $container_id:expr,
        color: $color:expr,
        width: $width:expr,
        height: $height:expr,
        positions: $position_call:tt[$($position_arguments:expr,)*]
    ) => {
        // Initialize container style
        let mut container_style = widget::canvas::Style::default();

        container_style.border = Some(0.0);
        container_style.border_color = Some(color::TRANSPARENT);
        container_style.color = Some($color);

        // Create container
        widget::Canvas::new()
            .with_style(container_style)
            .w_h($width, $height)
            .$position_call($($position_arguments,)*)
            .set($container_id, &mut $master.ui);
    };
}

macro_rules! gen_widget_button {
    (
        $master:ident,
        button_id: $button_id:expr,
        text_id: $text_id:expr,
        text_color: $text_color:expr,
        text_font_size: $text_font_size:expr,
        width: $width:expr,
        value_top: $value_top:expr,
        value: $value:expr,
        positions: ($($position_call:tt[$($position_arguments:expr,)*]),+)
    ) => {
        // Initialize button style
        let button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

        // Create rectangle for button
        widget::RoundedRectangle::styled(
            [$width, BUTTON_HEIGHT],
            BUTTON_BORDER_RADIUS,
            button_style
        )
            $(.$position_call($($position_arguments,)*))+
            .set($button_id, &mut $master.ui);

        // Create text style for button text
        let mut text_style = widget::text::Style::default();

        text_style.font_id = Some(Some($master.fonts.bold));
        text_style.color = Some($text_color);
        text_style.font_size = Some($text_font_size);

        // Append button text
        widget::Text::new($value)
            .with_style(text_style)
            .mid_top_with_margin_on($button_id, $value_top)
            .set($text_id, &mut $master.ui);
    };
}

macro_rules! gen_widget_label_styled {
    (
        $master:ident,
        text_id: $text_id:expr,
        value: $value:expr,
        font_size: $font_size:ident,
        font_weight: $font_weight:ident,
        positions: $position_call:tt[$($position_arguments:expr,)*]
    ) => {
        // Initialize text style
        let mut text_style = widget::text::Style::default();

        text_style.font_id = Some(Some($master.fonts.$font_weight));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some($font_size);

        // Create text
        widget::Text::new($value)
            .with_style(text_style)
            .$position_call($($position_arguments,)*)
            .set($text_id, &mut $master.ui);
    };
}

macro_rules! gen_widget_label_form {
    (
        $master:ident,
        text_id: $text_id:expr,
        value: $value:expr,
        positions: $position_call:tt[$($position_arguments:expr,)*]
    ) => {
        gen_widget_label_styled!(
            $master,
            text_id: $text_id,
            value: $value,
            font_size: MODAL_TEXT_FONT_SIZE,
            font_weight: regular,
            positions: $position_call[$($position_arguments,)*]
        );
    };
}

macro_rules! gen_widget_button_navigate {
    (
        $master:ident,
        button_less_id: $button_less_id:expr,
        button_less_text_id: $button_less_text_id:expr,
        button_more_id: $button_more_id:expr,
        button_more_text_id: $button_more_text_id:expr,
        value_id: $value_id:expr,
        value: $value:expr,
        positions: $position_call:tt[$($position_arguments:expr,)*]
    ) => {
        // Create less button
        gen_widget_button!(
            $master,
            button_id: $button_less_id,
            text_id: $button_less_text_id,
            text_color: color::BLACK,
            text_font_size: MODAL_BUTTON_NAVIGATE_FONT_SIZE,
            width: MODAL_BUTTON_NAVIGATE_WIDTH,
            value_top: MODAL_BUTTON_NAVIGATE_VALUE_MARGIN_TOP,
            value: MODAL_BUTTON_NAVIGATE_VALUE_DECREASE,
            positions: ($position_call[$($position_arguments,)*])
        );

        // Initialize text style for value
        let mut value_style = widget::text::Style::default();

        value_style.font_id = Some(Some($master.fonts.regular));
        value_style.color = Some(color::WHITE);
        value_style.font_size = Some(MODAL_TEXT_FONT_SIZE);

        // Create text for value
        widget::Text::new($value)
        .with_style(value_style)
        .right_from($button_less_id, MODAL_BUTTON_NAVIGATE_PADDING_INNER)
        .y_relative(0.0)
        .set($value_id, &mut $master.ui);

        // Create more button
        gen_widget_button!(
            $master,
            button_id: $button_more_id,
            text_id: $button_more_text_id,
            text_color: color::BLACK,
            text_font_size: MODAL_BUTTON_NAVIGATE_FONT_SIZE,
            width: MODAL_BUTTON_NAVIGATE_WIDTH,
            value_top: MODAL_BUTTON_NAVIGATE_VALUE_MARGIN_TOP,
            value: MODAL_BUTTON_NAVIGATE_VALUE_INCREASE,

            positions: (
                right_from[
                    $value_id,
                    MODAL_BUTTON_NAVIGATE_PADDING_INNER,
                ],
                y_relative[
                    -3.0,
                ]
            )
        );
    };
}
