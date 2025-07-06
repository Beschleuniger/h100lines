#![allow(unused_imports)]
#![allow(non_snake_case)]


use std::default;
use std::f32::INFINITY;
use std::sync::atomic;

use iced::widget::{
    button, column, container, row, text, Column, Scrollable, Text

};
use iced::{Element, Font};
use iced::Settings;

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Insert(String),
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

    fn insert(&mut self, test_str: String);

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


impl<'a> EntryManager for State {
    
    fn sort() {
        todo!()
    }

    
    fn insert(&mut self, test_str: String) {
        
        self.current_imported += 1;
        self.total_imported += 1;

        self.entries.push(
            Entry {
                title: test_str.to_string(), id: self.total_imported, placement: self.current_imported, ..Default::default()
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
            Message::Insert(ins) => state.insert(ins),
        }

        let s = state.current_imported ;
        println!("{s}");

    }

    fn view<'a>(state: &'a State) -> Element<'a, Message> {
        
        
        
        let inc = button(
            text(
                state.current_imported
            )
        ).on_press(Message::Increment);


        let dec = button(
            text(
                state.current_imported
            )
        ).on_press(Message::Decrement);

        let add = button(
            text(
                state.current_imported
            )
        ).on_press(Message::Insert(String::from("test")));


        let m_col;
        if state.total_imported > 0 {
            m_col = state.entries.iter().map(|s| Text::new(&s.title)).fold(Column::new(), |col, text| col.push(text));
        } else {
            m_col = column![text(String::from("lol"))];
        }
            
        

        let interface = column![inc, dec, add];
        
        let irow = row!(interface, m_col);

        println!("{:#?}",  state);

        container(irow)
                                    .center_x(INFINITY)
                                    .style(container::rounded_box)
                                    .padding(10)
                                    .into()
    }
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
