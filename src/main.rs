mod config;

use std::fs::{File, ReadDir};
use std::io::{BufRead, BufReader};
use std::path::Path;
use colored::Colorize;
use crate::config::Config;

fn main() {
    let config_ref = &Config::parse(std::env::args().collect());

    for filename in &config_ref.filenames {
        let path = Path::new(&filename);
        if !path.exists() {
            println!("file {} not exist", &filename.yellow());
        } else if path.is_file() {
            let file = File::open(path).unwrap();
            deal_with_file(&file, config_ref, &filename);
        } else if path.is_dir() {
            let read_dir = std::fs::read_dir(path).unwrap();
            deal_with_dir(read_dir, &config_ref);
        }
    }
}

fn deal_with_file(file_ref: &File, config_ref: &Config, filename: &str) {
    let reader = BufReader::new(file_ref);
    for (num, line) in reader.lines().enumerate() {
        let line_str = match line {
            Ok(str) => str,
            Err(err) => {
                println!("{} on reading file: {}, {}",
                         "ERROR".red(), filename.yellow(), err);
                return;
            }
        };
        if line_str.contains(&config_ref.target) {
            println!("file {} contains {} in line {}",
                     filename.yellow(), &config_ref.target.blue(), (num + 1).to_string().green());
        }
    }
}

fn deal_with_dir(read_dir: ReadDir, config_ref: &Config) {
    for dir_entry in read_dir {
        let entry_path = &dir_entry.unwrap().path();
        let sub_filename = entry_path.to_str().unwrap();
        if entry_path.is_file() {
            let file = File::open(entry_path).unwrap();
            deal_with_file(&file, config_ref, sub_filename);
        } else if entry_path.is_dir() {
            if config_ref.recursion {
                let path = Path::new(&entry_path);
                let sub_read_dir = std::fs::read_dir(path).unwrap();
                deal_with_dir(sub_read_dir, config_ref);
            } else { println!("{} is directory. skipped", sub_filename.yellow()) }
        }
    }
}
