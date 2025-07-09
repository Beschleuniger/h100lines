
use std::path::PathBuf;

use crate::helper::*;


#[derive(Debug, Clone)]
pub enum Message {
    InsertNA(String, String),
    InsertNAY(String, String, String),
    InsertFromYT(String),
    DeleteLast,
    DeleteAt(usize),
    MoveUp(usize),
    MoveDown(usize),
    DownloadComplete(String),
}












#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Entry{
    id: usize,
    pub placement: usize,
    pub title: String,
    pub artist: String,
    pub link: String,
    pub thumbnail: Option<PathBuf>,
}

impl Default for Entry {
    fn default() -> Self {
        Self { 
            id: 0,
            placement: 0,
            title: String::new(), 
            artist: String::new(),
            link: String::new(),
            thumbnail: None,
        }
    }
}


pub trait EntryManager {
    fn resetIDS(&mut self);

    fn insert(&mut self, name: &String, artist: &String);

    fn insertAt();

    fn delete(&mut self);
    
    fn deleteAt();

    fn moveUp(&mut self, index: usize);

    fn moveDown(&mut self, index: usize);
}





#[derive(Debug, Clone)]
pub struct State {
    pub current_imported: usize,
    pub total_imported: usize,
    pub entries_cached: bool,
    pub entries: Vec<Entry>,
}




impl Default for State {
    fn default() -> Self {
        Self { current_imported: 0, total_imported: 0, entries_cached: false, entries: Vec::with_capacity(100) }
    }
}

impl<'a> EntryManager for State {
    
    
    fn resetIDS(&mut self) {
        let mut max = 0;
        
        for (index, item) in self.entries.iter_mut().enumerate() {
            item.placement = index;
            max = index;
        }

        self.current_imported = max;

    }
    
    fn insert(&mut self, name: &String, artist: &String) { //, link: Option<&String>
        
        if self.total_imported > u32::MAX as usize {
            return ();
        }

        self.current_imported += 1;
        self.total_imported += 1;

        // let true_link: String = match link {
        //     Some(L) => validateOrReplaceLink(*L, name, artist),
        //     None => searchLink(&name, &artist),
        // };

        self.entries.push(
            Entry {
                title: name.clone(),
                artist: artist.clone(),
                id: self.total_imported,
                placement: self.current_imported,
                ..Default::default()
            }
        );

    }

    fn insertAt() {
        todo!()
    }

    fn delete(&mut self) {
        self.entries.pop();
    }
    
    fn deleteAt() {
        todo!()
    }

    fn moveUp(&mut self, index: usize) {

        if index == 0 {
            println!("Abort! Index: {} == 0", index);
            return;
        }

        if let Some(_) = self.entries.get(index) {
            self.entries.swap(index, index - 1);
        }
        
    }

    fn moveDown(&mut self, index: usize) {
        if index == self.current_imported {
            println!("Abort! Index: {} == Imported: {}", index, self.current_imported);
            return;
        }

        if let Some(_) = self.entries.get(index) {
            self.entries.swap(index, index + 1);
        }
    }
}

