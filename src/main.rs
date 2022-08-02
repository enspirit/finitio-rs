use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};

use finitio::fio;
use finitio::schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmd_parse()
}

fn cmd_parse() -> Result<(), Box<dyn std::error::Error>> {
    // Parse FIO file
    let mut idocs: Vec<fio::Schema> = Vec::new();
    let idoc = fio::parse_schema("@import finitio/data").map_err(|e| format!("{}", e))?;
    idocs.push(idoc);

    println!("{:?}", idocs);

    Ok(())
}
