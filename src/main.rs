use finitio::fio;
use finitio::schema;
use finitio::js;

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
            let main_fio = fio::parse_schema(&contents[..]).expect("Syntax error");
            fios.push(main_fio);

            let res = schema::Schema::from_fio(fios.iter());
            match res {
                Ok(_) => println!("Your schema is valid!"),
                Err(err) => panic!("Your schema is invalid: {}", err),
            }
            Ok(())
        },
    }
}
