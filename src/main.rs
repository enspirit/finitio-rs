use finitio::fio;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd_parse()
}

const TEST_SCHEMA: &str = r#"
@import finitio/data

Number = .Number
Integer = Number

NumberSeq = [Number]
NumberSet = {Number}
"#;

fn cmd_parse() -> Result<(), Box<dyn std::error::Error>> {
    // Parse FIO file
    let mut fios: Vec<fio::Schema> = Vec::new();
    let fio = fio::parse_schema(TEST_SCHEMA).map_err(|e| format!("{}", e))?;
    fios.push(fio);

    println!("{:?}", fios);

    Ok(())
}
