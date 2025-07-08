
use std::f32::INFINITY;

use iced::widget::{
    button, column, container, row, text, Button, Column, Container, Scrollable, Text

};
use iced::{Color, Element, Font};

use crate::state::*;
use crate::styling::{bgStyle, BORDER_RADIUS};



impl State {

    pub fn update(state: &mut State, message: Message) {
        
        match message {
            Message::InsertNA(name, artist) => todo!(),
            Message::InsertNAY(name, artist, link) => state.insert(name, artist, Some(link)),
            Message::InsertFromYT(playlist) => todo!(),
            Message::MoveUp(index) => state.moveUp(index),
            Message::MoveDown(index) => state.moveDown(index),
            Message::DeleteLast => state.delete(),
            Message::DeleteAt(_) => todo!(),
        }

        state.resetIDS();

        let s = state.current_imported;
        println!("{s}");

    }

    pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
        
        // Message Buttons
        

        let add: iced::widget::Button<'_, Message> = button(
            iced::widget::Text::<'_, iced::Theme, iced::Renderer>::new("+")
        ).on_press(Message::InsertNAY(format!("Moonstruck Dance {}", state.total_imported), String::from("Hatsunetsumiko's"), String::from("https://www.youtube.com/watch?v=es4MFjdtpYo")));


        let rem: iced::widget::Button<'_, Message> = button(
            iced::widget::Text::<'_, iced::Theme, iced::Renderer>::new("-")
        ).on_press(Message::DeleteLast);


        // Display Lists

        // let m_col;
        // if state.total_imported > 0 {
        //     m_col = state.entries.iter().map(|s| Text::new(&s.title)).fold(Column::new(), |col, text| col.push(text));
        // } else {
        //     m_col = column![text(String::from("lol"))];
        // }
            
        

        let interface = column![add, rem];
        

        

        
        
        
        let content = state.entries.iter().enumerate().fold(
            Column::new().spacing(8).padding(8),
            |col, (i, entry)| {
                col.push(entry_view(entry, i))
            },
        );
        
        let scroll = iced::widget::scrollable(content);
        
        let irow = row!(interface, scroll);
 

        container(irow)
                                .center_x(INFINITY)
                                .style(bgStyle())
                                .width(iced::Length::Fill)
                                .padding(10)
                                .into()
    }
}



fn entry_view(entry: &Entry, index: usize) -> Container<Message> {
    let placement = text(entry.placement.to_string()).width(30);

    let details = text(format!("{} - {}", entry.title, entry.artist))
        .width(iced::Length::Fill)
        .size(16);

    // let thumb = if let Some(ref path) = entry.thumbnail {
    //     iced::image(image::Handle::from_path(path)).width(60).height(60)
    // } else {
    //     container(text("No image").size(12))
    //         .width(60)
    //         .height(60)
    //         .align_x(alignment::Horizontal::Center)
    //         .align_y(alignment::Vertical::Center)
    //         .into()
    // };

    let buttons = Column::new()
        .spacing(2)
        .push(
            button(text("▲")).on_press(Message::MoveUp(index)),
        )
        .push(
            button(text("▼")).on_press(Message::MoveDown(index)),
        );

    let entry_row = iced::widget::Row::new()
        .spacing(10)
        .push(buttons)
        .push(placement)
        .push(details);

    container(entry_row)
        .padding(10)
        .style(|_: &_| container::Style {
            border: iced::Border { color: Color::default(), width: 1.0, radius: iced::border::Radius { 
                top_left: BORDER_RADIUS,
                top_right: BORDER_RADIUS,
                bottom_right: BORDER_RADIUS,
                bottom_left: BORDER_RADIUS,
            }},
            background: Some(iced::Background::Color(
                iced::Color::from_rgb8(220, 220, 250), // Lavender color
            )),
            ..Default::default()
        })
        .width(iced::Length::Fill)
}

