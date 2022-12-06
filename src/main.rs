#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, OpenOptions},
    io::prelude::*,
};

mod cli;
mod gui;

// 255 is the 'null' without having to make everything a Some()
// ignored by print, math, might spit out an error
pub const BLANK: u8 = u8::MAX;

#[derive(Deserialize, Serialize)]
pub struct Data {
    classes: Vec<Class>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Class {
    name: String,
    entries: Vec<Entry>,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Deserialize, Serialize)]
struct Entry(DateTime<Local>, u8);

fn main() {
    let file = read_to_string("./data.ron").unwrap();

    let mut data: Data = ron::from_str(&file).unwrap();

    let gui_debug = true;

    if gui_debug {

        gui::test_gui(data);

    } else {
    
        cli::parse(&mut data);

        data.save_to_file()

    }
}

impl Data {
    fn save_to_file(self) {
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("./data.ron")
            .unwrap();

        let config = PrettyConfig::new()
            .struct_names(true)
            .enumerate_arrays(true);

        f.write_all((to_string_pretty(&self, config)).unwrap().as_bytes())
            .unwrap();
    }

    fn make_naive_grid(&self) -> Vec<Vec<u8>> {
        let mut naive_grid = Vec::new();

        for class in self.classes.clone() {
            let mut temporary_class = Vec::new();

            for entry in class.entries {
                temporary_class.push(entry.1);
            }

            naive_grid.push(temporary_class)
        }

        naive_grid
    }

    fn create_blank_class(&mut self, class_name: String) {
        let existing_entries = self.classes[0].entries.clone();
        let mut blank_entries = Vec::new();

        for entry in existing_entries {
            blank_entries.push(Entry(entry.0, BLANK))
        }

        self.classes.push(Class {
            name: class_name,
            entries: blank_entries,
        });
    }

    fn create_blank_row(&mut self, timestamp: DateTime<Local>) {
        for i in self.classes.iter_mut() {
            i.entries.push(Entry(timestamp, BLANK));
            i.entries.sort();
        }
    }

    #[allow(dead_code)]
    fn transpose(&self) -> Vec<Vec<u8>> {
        let mut r = Vec::new();
        let table = self.make_naive_grid();

        for n in 0..table[0].len() {
            let mut temp = Vec::new();

            for i in table.clone() {
                temp.push(i[n]);
            }

            r.push(temp.clone());
        }

        r
    }

    fn set(&mut self, x: usize, y: usize, input: u8) {
        self.classes[x].entries[y].1 = input;
    }

    fn rename_class(&mut self, old_class_name: String, new_class_name: String) {
        for class in self.classes.iter_mut() {
            if class.name == old_class_name {
                class.name = new_class_name.clone();
            }
        }
    }
}
