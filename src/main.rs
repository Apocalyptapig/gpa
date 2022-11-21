use chrono::prelude::*;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Data(Vec<Class>);

#[derive(Clone, Deserialize, Serialize)]
struct Class {
    name: String,
    y: Vec<Entry>
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Deserialize, Serialize)]
struct Entry(DateTime<Local>, u8); 

fn main() {
    let file = read_to_string("./data.ron").unwrap();

    let mut data: Data = ron::from_str(&file).unwrap();

    cli::parse(&mut data);

    save_to_file(data);
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

        for i in self.0.clone() {
            let mut temp = Vec::new();

            for i in i.y {
                temp.push(i.1);
            }

            r.push(temp)
        }
        
        r
    }

    fn new(&mut self, name: String, input: u8, timestamp: DateTime<Local>) {
        self.0.push(Class { name, y: vec![Entry(timestamp, input)] });
    }

    fn add_y(&mut self, timestamp: DateTime<Local>) {
        for i in self.0.iter_mut() {
            i.y.push(Entry(timestamp, 255));
            i.y.sort();
        }
    }

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
        self.0[x].y[y].1 = input;
    }
}

mod cli {
    use chrono::prelude::*;
    use clap::{Args, Parser, Subcommand};
    use crate::Data;
    use term_grid::*;

    impl Data {
        fn print(&self) {
            let table = self.organize();

            let mut grid = Grid::new(GridOptions {
                filling:    Filling::Text(" | ".to_string()),
                direction:  Direction::TopToBottom,
            });

            grid.add(Cell::from("".to_string()));

            for i in 0..table[0].len() {
                grid.add(Cell::from(i.to_string()));
            }
            
            for (n, i) in table.iter().enumerate() {
                grid.add(Cell::from(format!("_{}_", n.to_string())));

                for j in i {
                    let s = match j {
                        255 => "[!]".to_string(),
                        _ => j.to_string()
                    };
                    grid.add(Cell::from(s));
                }
            }

            let r = grid.fit_into_columns(table.len() + 1);
            print!("{r}");
        }
    }

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Cli {
        #[command(subcommand)]
        command: Commands,

        /// excessive printing
        #[arg(short, long)]
        verbose: bool,
    }

    #[derive(Subcommand)]
    enum Commands {
        #[command(alias = "s")]
        Set(Set),

        #[command(alias = "n")]
        New(New),

        #[command(visible_alias = "d", alias = "disp")]
        Display(Display),
    }
    
    #[derive(Args)]
    struct Set {
        x: usize,

        y: usize,

        input: u8,
    }

    #[derive(Args)]
    struct Display;

    #[derive(Args)]
    struct New;

    pub fn parse(data: &mut Data) {
        let cli = Cli::parse();

        let timestamp = Local::now();

        match cli.command {
            Commands::Set(set) => {
                data.set(set.x, set.y, set.input);
            },

            Commands::New(_) => {
                data.add_y(timestamp);
            },

            Commands::Display(_) => (),
        }
        data.print();
    }
}

// todo

// log exists
    // class given, input given => write to class
        // class exists, write to it
        // class does not exist, write it

    // class given => query class
        // class exists, return value
        // class does not exist, PANIC

// log does not exist
    // class given, input given => create log with (class, input) input
    // _ => PANIC