#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::prelude::*;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, OpenOptions, File},
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
    let file = read_to_string("./data.ron");
    
    let mut data = match file {
        Ok(file) => ron::from_str(&file).unwrap(),
        Err(_) => Data::new()
    };

    let gui_debug = false;

    if gui_debug {
        gui::test_gui(data);
    } else {
        cli::parse(&mut data);

        data.save_to_file()
    }
}

impl Data {
    fn new() -> Self {
        Self {
            classes: vec!(
                Class {
                    name: "default".to_string(),
                    entries: vec!(
                        Entry(
                            Local::now(),
                            255
                        )
                    )
                }
            )
        }
    }

    fn save_to_file(self) {
        let data_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("./data.ron");

        let mut open_file = match data_file {
            Ok(file) => file,
            Err(_) => {
                File::create("./data.ron")
                    .expect("file creation messed up. what did you do?")
            }
        };

        let config = PrettyConfig::new()
            .struct_names(true)
            .enumerate_arrays(true);

        open_file.write_all((to_string_pretty(&self, config)).unwrap().as_bytes())
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

    fn new_blank_class(&mut self, class_name: String) {
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

    fn new_blank_row(&mut self, timestamp: DateTime<Local>) {
        for i in self.classes.iter_mut() {
            i.entries.push(Entry(timestamp, BLANK));
            i.entries.sort();
        }
    }

    fn set(&mut self, x: usize, y: usize, input: u8) {
        self.classes[x].entries[y].1 = input;
    }
}

trait Transpose {
    fn transpose(&self) -> Vec<Vec<u8>>;
}

impl Transpose for Vec<Vec<u8>> {
    fn transpose(&self) -> Vec<Vec<u8>> {
        let mut r = Vec::new();

        for n in 0..self[0].len() {
            let mut temp = Vec::new();

            for i in self.clone() {
                temp.push(i[n]);
            }

            r.push(temp.clone());
        }

        r
    }
}