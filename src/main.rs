use std::env;
use std::fs;

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

fn print_file(file_path: String) {
    let contents = fs::read_to_string(file_path.clone()).unwrap();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let extension = file_path.split(".").collect::<Vec<&str>>();

    let syntax = ps.find_syntax_by_extension(extension[1]).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in LinesWithEndings::from(&contents) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }       
    // Clear the formatting
    println!("\x1b[0m");
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

