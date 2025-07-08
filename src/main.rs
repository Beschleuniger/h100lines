#![allow(unused_imports)]
#![allow(non_snake_case)]


use std::default;
use std::sync::atomic;

use iced::Settings;
use iced::Font;

mod state;
use state::*;

mod styling;
mod layout;
mod helper;





fn main() -> iced::Result {

    let settings = Settings {
        id: Some("Cock".to_string()),
        fonts: vec![],
        default_font: Font::with_name("Century Gothic"),
        default_text_size: 25.0.into(),
        antialiasing: true,    
    };

    iced::application("Cock", State::update, State::view)
        .settings(settings)
        .window_size(iced::Size::new(800.0, 600.0))
        .run()
}
