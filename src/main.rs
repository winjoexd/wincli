#![allow(unused)]

use clap::Parser;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use indicatif::ProgressBar;
use std::{thread, time};
use log::{info, warn, debug};

#[derive(Debug, Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn find_matches(mut reader: BufReader<File>, pattern: &str, mut writer: impl std::io::Write) {
    for line in reader.lines() {
        let l = line.as_ref().unwrap();
        if l.contains(&pattern) {
            writeln!(writer, "{}", l);
        }
    }
}

#[test]
fn find_a_match() -> std::io::Result<()> {
    let mut result = Vec::new();
    let f = File::open("test_find_a_match.txt")?;
    let reader = BufReader::new(f);
    find_matches(reader, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Cli::parse();
   
    info!("pattern = {}", args.pattern);
    info!("path = {:?}", args.path);

    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        thread::sleep(time::Duration::from_millis(30));
        pb.inc(1);
    }
    pb.finish();
    
    find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}
