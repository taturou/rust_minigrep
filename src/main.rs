use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("Searching for \"{}\" in file \"{}\"", query, filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contentes = String::new();
    f.read_to_string(&mut contentes).expect("something went wrong reading the file");

    println!("With the text:\n{}", contentes);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
