#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
   
    println!("pattern = {}", args.pattern);
    println!("path = {:?}", args.path);

    let f = File::open(&args.path)?;
    let mut reader = BufReader::new(f);
    
    for line in reader.lines() {
        if line.as_ref().unwrap().contains(&args.pattern) {
            println!("{:?}", line);
        }
    }

    Ok(())
}
