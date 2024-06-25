use std::env;
use std::fs;

extern crate colored;
use colored::*;

fn print_file(file_path: String) {
    let contents = fs::read_to_string(file_path).unwrap();
    print!("{}", String::from(contents).italic().red());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("File: {}", args[1]);
        println!("---------------{}--------------\n", args[1]);
        print_file(args[1].clone());
        println!("\n--------------------End-of-File--------------------");
    } else {
        println!("Dog is a 'cat' implementation with barks instead of meow");
        println!("Using example: dog [file]");
    }
}
