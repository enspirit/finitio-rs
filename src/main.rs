use finitio::fio;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd_parse()
}

fn cmd_parse() -> Result<(), Box<dyn std::error::Error>> {
    // Parse FIO file
    let mut fios: Vec<fio::Schema> = Vec::new();
    let fio = fio::parse_schema("@import finitio/data").map_err(|e| format!("{}", e))?;
    fios.push(fio);

    println!("{:?}", fios);

    Ok(())
}
