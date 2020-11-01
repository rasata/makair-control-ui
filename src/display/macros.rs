// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_load_font {
    ($family:expr, $name:expr) => {
        conrod_core::text::Font::from_bytes(
            inflate_bytes_zlib(
                &EmbeddedFonts::get(&format!("{}/{}.ttf.zz", $family, $name)).unwrap(),
            )
            .unwrap(),
        )
        .unwrap()
    };
}

macro_rules! gen_load_image_reverse {
    ($name:expr, $width:ident) => {
        reverse_rgba(
            &load_from_memory(
                EmbeddedImages::get(&format!("{}.png", $name))
                    .unwrap()
                    .to_mut(),
            )
            .unwrap()
            .into_rgba()
            .into_raw(),
            $width,
        )
    };
}

macro_rules! gen_draw_cached_image {
    ($display:ident <= $logo_rgba:ident[$width:ident, $height:ident]) => {
        // Notice: build the raw image directly using the texture internals, as to avoid cloning \
        //   the raw image bytes at every refresh.
        glium::texture::Texture2d::new(
            &$display.0,
            glium::texture::RawImage2d {
                data: Cow::Borrowed(&*$logo_rgba),
                width: $width,
                height: $height,
                format: glium::texture::ClientFormat::U8U8U8U8,
            },
        )
        .unwrap()
    };
}

macro_rules! gen_widget_impls {
    ($($control:tt -> $widget:ident $([$lifetime:lifetime])*),+,) => {
        pub enum ControlWidgetType<'a> {
            $(
                $control($widget::Config<$($lifetime)*>),
            )+
        }

        impl<'a> ControlWidget<'a> {
            pub fn render(&mut self, widget_type: ControlWidgetType<'a>) -> f64 {
                match widget_type {
                    $(
                        ControlWidgetType::$control(config) => $widget::render(self, config),
                    )+
                }
            }
        }
    }
}

macro_rules! gen_ui_events_telemetry_settings_clicks {
    ($interface:ident, $({$name:expr, $settings_state:expr, $widget_ids:expr}),+,) => {
        $(
            let clicks = DisplayUIEvents::count_clicks(
                $interface,
                &$widget_ids,
            );

            for _ in 0..clicks {
                debug!("pressed the {} widget once", $name);

                $settings_state.toggle();
            }
        )+
    }
}

macro_rules! gen_ui_events_modal_settings_clicks {
    (
        $interface:ident,
        $ids:ident,
        $events:ident,

        $({
            $name:expr,
            $type:tt,
            $settings_state:expr,

            $({
                $action:expr,
                $field_name:expr,
                $widget_ids:expr
            }),*
        }),+,
    ) => {
        $(
            if $settings_state == &DisplayRendererSettingsState::Opened {
                // Handle clicks on the close button
                for _ in 0..DisplayUIEvents::count_clicks(
                    $interface,
                    &[
                        $ids.modal_validate,
                        $ids.modal_validate_text,
                    ],
                ) {
                    debug!("pressed the {} settings close button once", $name);

                    $settings_state.toggle();
                }

                // Handle clicks on dynamic form elements
                $(
                    for _ in 0..DisplayUIEvents::count_clicks(
                        $interface,
                        &$widget_ids,
                    ) {
                        debug!("pressed the {} settings {} field button once", $name, $field_name);

                        $events.push(ChipSettingsEvent::$type(
                            $action,
                        ));
                    }
                )*
            }
        )+
    }
}
