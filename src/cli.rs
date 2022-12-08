use crate::{Data, BLANK};
use chrono::prelude::*;
use clap::{Args, Parser, Subcommand};
use term_grid::*;

impl Data {
    fn print(&self, verbose: bool) {
        let table = self.make_naive_grid();

        let mut grid = Grid::new(GridOptions {
            filling: Filling::Text(" | ".to_string()),
            direction: Direction::TopToBottom,
        });

        grid.add(Cell::from("".to_string()));

        for i in 0..table[0].len() {
            grid.add(Cell::from(i.to_string()));
        }

        for (n, i) in table.iter().enumerate() {
            let cell = match verbose {
                true => Cell::from(format!("_{}_", n)),
                false => Cell::from(self.classes[n].name.clone()),
            };

            grid.add(cell);

            for j in i {
                let s = match j {
                    &BLANK => "[!]".to_string(),
                    _ => j.to_string(),
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

    #[command(alias = "r")]
    Rename(Rename),

    #[command(alias = "nc")]
    NewClass(NewClass),
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

#[derive(Args)]
struct Rename {
    from: String,
    into: String,
}

#[derive(Args)]
struct NewClass {
    name: String,
}

pub fn parse(data: &mut Data) {
    let cli = Cli::parse();

    let timestamp = Local::now();

    match cli.command {
        Commands::Set(set) => {
            data.set(set.x, set.y, set.input);
        }

        Commands::New(_) => {
            data.new_blank_row(timestamp);
        }

        Commands::Display(_) => (),

        Commands::Rename(rename) => {
            for class in data.classes.iter_mut() {
                if class.name == rename.from {
                    class.name = rename.into.clone()
                }
            }
        }

        Commands::NewClass(new_class) => data.new_blank_class(new_class.name),
    }
    data.print(!cli.verbose);
}
