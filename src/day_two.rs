use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let filename = "input_day_two.txt";

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let str_contents: &str = &contents[..];
    let checksum: u32 = str_contents.lines().map( |line| {
            let items: Vec<u32> = line.split("\t").map(|item| item.parse::<u32>().unwrap()).collect();
            let item_ref = &items;
            let max = item_ref.into_iter().max().unwrap();
            let min = item_ref.into_iter().min().unwrap();
            return max - min;
    }).sum();

    println!("Checksum: {}", checksum);
}