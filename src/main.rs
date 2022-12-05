#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, OpenOptions},
    io::prelude::*, default,
};

mod cli;
mod gui;

// 255 is the 'null' without having to make everything a Some()
// ignored by print, math, might spit out an error
const DEFAULT: u8 = 255;

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
    /*
    let file = read_to_string("./data.ron").unwrap();

    let mut data: Data = ron::from_str(&file).unwrap();

    cli::parse(&mut data);

    save_to_file(data);
    */

    gui::test_gui();
}

fn save_to_file(data: Data) {
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./data.ron")
        .unwrap();

    let config = PrettyConfig::new()
        .struct_names(true)
        .enumerate_arrays(true);

    f.write_all((to_string_pretty(&data, config)).unwrap().as_bytes())
        .unwrap();
}

impl Data {
    fn organize(&self) -> Vec<Vec<u8>> {
        let mut r = Vec::new();

        for i in self.classes.clone() {
            let mut temp = Vec::new();

            for i in i.entries {
                temp.push(i.1);
            }

            r.push(temp)
        }

        r
    }

    fn create_default_class(&mut self, class_name: String) {
        let default_entries = self.classes[0].entries;

        for entry in default_entries {
            entry.1 = DEFAULT
        }

        self.classes.push(Class {
            name: class_name,
            entries: default_entries 
        });
    }

    fn create_default_row(&mut self, timestamp: DateTime<Local>) {
        for i in self.classes.iter_mut() {
            i.entries.push(Entry(timestamp, DEFAULT));
            i.entries.sort();
        }
    }

    #[allow(dead_code)]
    fn transpose(&self) -> Vec<Vec<u8>> {
        let mut r = Vec::new();
        let table = self.organize();

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
