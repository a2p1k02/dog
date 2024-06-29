use std::fs;

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

fn output(file_path: String) {
    let contents = fs::read_to_string(file_path.clone()).unwrap();

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let extension = file_path.split(".").collect::<Vec<&str>>();

    let syntax = ps.find_syntax_by_extension(extension[1]).unwrap();

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    if extension[1] != "txt" {
        for i in 0..contents.split("\n").collect::<Vec<&str>>().len() - 1 {
            let line = LinesWithEndings::from(&contents).collect::<Vec<&str>>();
            let ranges: Vec<(Style, &str)> = h.highlight_line(line[i], &ps).unwrap();
            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
            print!("  {} | {}", i + 1, escaped);      
        }
    } else {
        print!("{}", contents);
    }

    // Clear the formatting
    println!("\x1b[0m");
}

pub fn help() {
    println!("Dog is a 'cat' implementation with barks instead of meow");
    println!("Using example: dog [file]");
}

pub fn print_file(args: Vec<String>) {
    println!("File: {}", args[1]);
    println!("---------------{}--------------\n", args[1]);
    output(args[1].clone());
    println!("\n--------------------End-of-File--------------------");
}

