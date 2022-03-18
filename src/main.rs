#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::{Error, ErrorKind};

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
    println!("===========================");

    let f = File::open(&args.path);
    match f{
        Err(error) => {
            return Err(Error::from(ErrorKind::NotFound));
        }
        _ => {}
    }

    let mut reader = BufReader::new(f.unwrap());
    
    for line in reader.lines() {
        let l = line.as_ref().unwrap();
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }

    Ok(())
}
