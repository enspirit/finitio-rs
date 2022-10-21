use finitio::fio;
use finitio::schema;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use std::fs;
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        #[arg(short, long)]
        filename: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Parse { filename } => {
            // Parse FIO file
            let contents = fs::read_to_string(filename)
                .expect("Should have been able to read the file");

            let mut fios: Vec<fio::Schema> = Vec::new();

            // Parse entry point
            let main_fio = fio::parse_schema(&contents[..]).expect("Syntax error");
            fios.push(main_fio);

            // Parse imports
            if !fios[0].imports.is_empty() {
                let base_dir = Path::new(filename).parent()
                    .ok_or("base_dir could not be determined from source")?;
                let mut included_files: HashSet<PathBuf> = HashSet::new();
                let mut includes = fios[0]
                    .imports
                    .iter()
                    .map(|p| (base_dir.join(&p.filename)))
                    .collect::<Vec<_>>();
                while !includes.is_empty() {
                    let include = includes.remove(0);
                    if included_files.contains(&include) {
                        continue;
                    }
                    included_files.insert(include.clone());
                    let contents = fs::read_to_string(include.clone())?;
                    let fio = fio::parse_schema(&contents[..]).expect("Syntax error");
                    let dir = include.parent().unwrap();
                    includes.extend(fio.imports.iter().map(|inc| dir.join(&inc.filename)));
                    fios.push(fio);
                }
            }

            let res = schema::Schema::from_fio(fios.iter());
            match res {
                Ok(_) => println!("Your schema is valid!"),
                Err(err) => panic!("Your schema is invalid: {}", err),
            }
            Ok(())
        },
    }
}
