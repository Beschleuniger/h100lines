#![allow(unused_imports)]
#![allow(non_snake_case)]


use std::default;
use std::f32::INFINITY;

use iced::widget::{
    button,
    column,
    text,
    Column,
    container,
    Scrollable,

};
use iced::{Element, Font};
use iced::Settings;

#[derive(Debug, Clone)]
enum Message<'a> {
    Increment,
    Decrement,
    Insert(&'a str),
}






#[derive(Debug, Clone)]
struct Entry<'a>{
    id: u16,
    placement: u16,
    title: &'a str,
    artist: &'a str,
    link: &'a str,
    thumbnail: Option<u8>,
}

impl Default for Entry<'_> {
    fn default() -> Self {
        Self { 
            id: 0,
            placement: 0,
            title: "", 
            artist: "",
            link: "",
            thumbnail: None
        }
    }
}



trait EntryManager {
    fn sort();

    fn insert(&mut self, test_str: &str);

    fn insertAt();

    fn delete();
    
    fn deleteAt();
}


#[derive(Debug, Clone)]
struct State<'a> {
    current_imported: u32,
    total_imported: u32,
    entries_cached: bool,
    entries: Vec<Entry<'a>>,
}

impl Default for State<'_> {
    fn default() -> Self {
        Self { current_imported: 0, total_imported: 0, entries_cached: false, entries: Vec::with_capacity(100) }
    }
}


impl<'a> EntryManager for State<'a> {
    
    fn sort() {
        todo!()
    }

    fn insert(&mut self, test_str: &str) {
    

    
        self.entries.push(
            Entry {
                title: test_str.to_string().as_str(), ..Default::default()
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


impl State<'_> {

    fn update(state: &mut State, message: Message) {
        
        match message {
            Message::Increment => state.current_imported += 1,
            Message::Decrement => state.current_imported -= 1,
            Message::Insert(ins) => state.insert(ins),
        }

        let s = state.current_imported ;
        println!("{s}");

    }

    fn view<'a>(state: &'a State) -> Element<'a, Message<'a>> {
        
        
        
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
        ).on_press(Message::Insert("test"));


        let interface = column![inc, dec, add];

        container(interface)
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
