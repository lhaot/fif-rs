use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    if 3 > args.len() {
        panic!("not enough arguments")
    }
    let target_string = &args[1];
    for filename_ref in &args[2..] {
        let path = Path::new(filename_ref);
        let file_ref = &File::open(path).unwrap();
        let reader = BufReader::new(file_ref);
        for (num, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.contains(target_string) {
                println!("file [{:?}] contains [{}] in line [{}]",
                         filename_ref, target_string, num + 1);
            }
        }
    }
}
