use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("ERROR: Invalid args.");
    }

    let file = BufReader::new(File::open(args.get(1).unwrap()).expect("ERROR: File not found."));
    let operations = file
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    for i in &operations {
        println!("{}", i);
    }
}
