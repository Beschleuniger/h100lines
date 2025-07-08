use iced::widget::container;



pub const BORDER_RADIUS: f32 = 8.0; 







// Fuck you
pub fn bgStyle() -> impl Fn(&iced::Theme) -> container::Style {
    |_t| container::Style { 
        text_color: Some(iced::Color::BLACK),
        background: Some(iced::Background::Color(iced::Color { r: 255.0, g: 60.0, b: 10.0, a: 100.0 })),
        border: iced::Border::default(),
        shadow: iced::Shadow::default() 
    }
}



