mod error;
mod scanner;
mod token;
mod token_type;
use crate::scanner::Scanner;
use error::*;

use std::{
    env,
    fs::{self},
    io::{self, stdout, Write},
    process,
};

use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        print!("Usage: jlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = fs::read_to_string(path)?;

    match run(buf) {
        Ok(_) => {}
        Err(mut m) => {
            m.report("".to_string());
            process::exit(65)
        }
    }

    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(_) => {
                    // Ignore: error was already reported
                }
            };
        } else {
            break;
        }
        print!("> ");
        stdout().flush();
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token)
    }

    Ok(())
}
