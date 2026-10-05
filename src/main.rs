use std::{
    env, fs,
    io::{self, BufRead, Write},
    process,
};

use crate::scanner::Scanner;

mod lox;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: lox [script]");
            process::exit(64); // command line usage error
        }
    };
}

fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read(path)?;
    let src = String::from_utf8(bytes).unwrap(); // TODO: map_err here to io Error
    run(&src);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        run(&line);
    }

    Ok(())
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}
