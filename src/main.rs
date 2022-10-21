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

            let res = fio::parse_schema(&contents[..])
                .map_err(|e| format!("{}", e));

            let adt = match res {
                Ok(adt) => {
                    println!("Syntax is valid!");
                    adt
                },
                Err(err) => panic!("Syntax error: {}", err),
            };

            let res = schema::Schema::from_fio(&adt);
            match res {
                Ok(_) => println!("Your schema is valid!"),
                Err(err) => panic!("Your schema is invalid: {}", err),
            }
            Ok(())
        },
    }
}
