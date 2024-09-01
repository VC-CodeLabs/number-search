// glen beane
// code labs number search
//
// compile:
// rustc -o numberSearch -C opt-level=3 main.rs
// (same optimization level cargo build --release uses)
//
// to run:
// numberSearch <input file path> <output file path>

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: numberSearch <input file path> <output file path>");
        return;
    }

    let file_path = &args[1];
    let out_path = &args[2];

    let mut output_file = match File::create(out_path) {
        Ok(f) => f,
        Err(e) => {
            eprint!("Error opening output file {}", e);
            return;
        }
    };

    let input_file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprint!("Error opening input file {}", e);
            return;
        }
    };


    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        match line {
            Ok(l) => {
                // skip empty string (for example, if input file ends in a newline)
                if l.chars().count() == 0 {
                    continue;
                }

                if let(Some(first), Some(last)) = (get_first_digit(l.clone()), get_last_digit(l)) {
                    writeln!(output_file, "{}{}", first, last).expect("Error writing to output file");
                }
            },
            Err(e) => eprintln!("Error reading line {}", e),
        }
    }
}

fn get_first_digit(s: String) -> Option<char> {
    for c in s.chars() {
        if c.is_numeric() {
            return Some(c);
        }
    }
    None
}

fn get_last_digit(s: String) -> Option<char> {
    for c in s.chars().rev() {
        if c.is_numeric() {
            return Some(c);
        }
    }
    None
}
