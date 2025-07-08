#![allow(unused_imports)]
#![allow(non_snake_case)]


use std::default;
use std::f32::INFINITY;
use std::sync::atomic;

use iced::widget::{
    button, column, container, row, text, Button, Column, Container, Scrollable, Text

};
use iced::{Element, Font};
use iced::Settings;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    InsertNA(String, String),
    InsertNAY(String, String, String),
    InsertFromYT(String),
    MoveUp(usize),
    MoveDown(usize)
}

// Fuck you
fn bgStyle() -> impl Fn(&iced::Theme) -> container::Style {
    |_t| container::Style { 
        text_color: Some(iced::Color::BLACK),
        background: Some(iced::Background::Color(iced::Color { r: 255.0, g: 60.0, b: 10.0, a: 100.0 })),
        border: iced::Border::default(),
        shadow: iced::Shadow::default() 
    }
}







#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Entry{
    id: u32,
    placement: u32,
    title: String,
    artist: String,
    link: String,
    thumbnail: Option<u8>,
}

impl Default for Entry {
    fn default() -> Self {
        Self { 
            id: 0,
            placement: 0,
            title: String::new(), 
            artist: String::new(),
            link: String::new(),
            thumbnail: None
        }
    }
}





trait EntryManager {
    fn sort();

    fn insert(&mut self, name: String, artist: String, link: Option<String>);

    fn insertAt();

    fn delete();
    
    fn deleteAt();
}


#[derive(Debug, Clone)]
struct State {
    current_imported: u32,
    total_imported: u32,
    entries_cached: bool,
    entries: Vec<Entry>,
}

impl Default for State {
    fn default() -> Self {
        Self { current_imported: 0, total_imported: 0, entries_cached: false, entries: Vec::with_capacity(100) }
    }
}


fn validateOrReplaceLink(link: String, name: &String, artist: &String) -> String {

    // search for link validity, otherwise call searchLink() 

    return link;
    searchLink(name, artist);
}

fn searchLink(name: &String, artist: &String) -> String {

    // search on youtube for song and get the link

    todo!()
}






impl<'a> EntryManager for State {
    
    fn sort() {
        todo!()
    }

    
    fn insert(&mut self, name: String, artist: String, link: Option<String>) {
        
        self.current_imported += 1;
        self.total_imported += 1;

        let true_link: String = match link {
            Some(L) => validateOrReplaceLink(L, &name, &artist),
            None => searchLink(&name, &artist),
        };

        self.entries.push(
            Entry {
                title: name,
                artist: artist,
                id: self.total_imported,
                placement: self.current_imported,
                link: true_link,
                ..Default::default()
            }
        );

        
    }

    fn insertAt() {
        todo!()
    }

    fn delete() {
        todo!()
    }
    
    fn deleteAt() {
        todo!()
    }
}


impl State {

    fn update(state: &mut State, message: Message) {
        
        match message {
            Message::Increment => state.current_imported += 1,
            Message::Decrement => state.current_imported -= 1,
            Message::InsertNA(name, artist) => todo!(),
            Message::InsertNAY(name, artist, link) => state.insert(name, artist, Some(link)),
            Message::InsertFromYT(playlist) => todo!(),
            Message::MoveUp(_) => todo!(),
            Message::MoveDown(_) => todo!(),
        }

        let s = state.current_imported ;
        println!("{s}");

    }

    fn view<'a>(state: &'a State) -> Element<'a, Message> {
        
        // Message Buttons
        

        let add: iced::widget::Button<'_, Message> = button(
            iced::widget::Text::<'_, iced::Theme, iced::Renderer>::new("add to list")
        ).on_press(Message::InsertNAY(String::from("Moonstruck Dance"), String::from("Hatsunetsumiko's"), String::from("https://www.youtube.com/watch?v=es4MFjdtpYo")));



        // Display Lists

        // let m_col;
        // if state.total_imported > 0 {
        //     m_col = state.entries.iter().map(|s| Text::new(&s.title)).fold(Column::new(), |col, text| col.push(text));
        // } else {
        //     m_col = column![text(String::from("lol"))];
        // }
            
        

        let interface = column![ add];
        

        

        
        
        
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
        // .style(|_: &_| container:: {
        //     border_radius: 10.0,
        //     background: Some(iced::Background::Color(
        //         iced::Color::from_rgb8(230, 230, 250), // Lavender color
        //     )),
        //     ..Default::default()
        // })
        .width(iced::Length::Fill)
}




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
