use std::env;

mod printer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        printer::print_file(args);
    } else {
        printer::help();
    }
}

