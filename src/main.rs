#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use indicatif::ProgressBar;

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
    let reader = BufReader::new(f);

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        pb.inc(1);
    }
    pb.finish();
    
    for line in reader.lines() {
        let l = line.as_ref().unwrap();
        if l.contains(&args.pattern) {
            println!("{}", l);
        }
    }

    Ok(())
}
