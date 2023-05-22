use std::io;
use std::{fs::File, println};

use clap::Parser;

mod args;
mod commands;

fn main() {
    let mut cli = args::Args::parse();
    let mut counter = commands::Counter::new();
    let mut output = Vec::new();
    // 16kb buffer size
    let buf_size = 16 * 1024;

    if let Some(file) = &cli.file {
        let file = File::open(file).expect("Unable to open file");
        counter
            .read(&file, buf_size)
            .expect("an error happened while counting");
    } else {
        counter
            .read(io::stdin().lock(), buf_size)
            .expect("an error happened while counting");
    }

    // Set default options if none are provided
    if !cli.lines && !cli.words && !cli.bytes && !cli.chars {
        cli.words = true;
        cli.bytes = true;
        cli.lines = true;
    }

    if cli.lines {
        output.push(format!("{}", counter.lines));
    }

    if cli.words {
        output.push(format!("{}", counter.words));
    }

    if cli.chars {
        output.push(format!("{}", counter.chars));
    }

    if cli.bytes {
        output.push(format!("{}", counter.bytes));
    }

    if let Some(file) = &cli.file {
        output.push(file.to_string());
    }

    println!("{}", output.join(" "));
}
