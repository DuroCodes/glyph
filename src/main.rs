use std::io::{self, Write};

use glyph::Glyph;

mod glyph;
mod operations;

fn main() {
    let mut interpreter = Glyph::new();
    let mut input = String::new();

    println!("Welcome to Glyph!");
    println!("Type 'exit' or 'quit' to exit the program.\n");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        input.clear();

        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        if ["exit", "quit"].contains(&input.to_lowercase().trim()) {
            break;
        }

        let ops = Glyph::parse(&input);
        interpreter.evaluate_sequence(ops);

        for stack in &interpreter.stack {
            println!("{stack:?}");
        }
    }
}
