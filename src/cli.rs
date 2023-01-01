use crate::{Data, BLANK, Transpose};
use chrono::prelude::*;
use clap::{Args, Parser, Subcommand};
use comfy_table::*;
use comfy_table::presets::UTF8_FULL;

impl Data {
    fn print(&self, verbose: bool) {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80);

        let mut header = vec![Cell::new("")];

        for (n, class) in self.classes.iter().enumerate() {
            let column_header = match verbose {
                true => class.name.clone(),
                false => n.to_string()
            };

            header.push(
                Cell::new(column_header)
                    .add_attribute(Attribute::Bold)
            )
        }

        table.set_header(header);

        let grid = self.make_naive_grid().transpose();

        for (n, row) in grid.iter().enumerate() {
            let row_header = match verbose {
                true => self.classes[0].entries[n].0.date_naive().to_string(),
                false => n.to_string()
            };
            let mut cells = vec![Cell::from(row_header)];

            for item in row {
                if *item == BLANK {
                    cells.push(
                        Cell::from("[!]")
                            .fg(Color::Red)
                    )
                } else {
                    let color = match item {
                        95.. => Color::Green,
                        _ => Color::Reset
                    };

                    cells.push(Cell::from(item).fg(color))
                }
            }

            table.add_row(cells);
        }

        println!("{table}");
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

        Commands::NewClass(new_class) => {
            data.new_blank_class(new_class.name)
        }
    }

    data.print(cli.verbose);
}
