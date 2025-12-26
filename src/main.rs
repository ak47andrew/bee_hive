mod translator;
mod cli;

use std::process;
use clap::Parser;
use crate::cli::Cli;
use crate::translator::tokenizer::{Expr, tokenize};

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).unwrap();

    // cleanup.rs
    let lines = translator::cleanup::split_statements(&content);

    // tokenizer.rs

    let asts: Vec<Expr> = lines.iter().map(|x| tokenize(x.clone())).map(
        |x| match x {
            Ok(expr) => expr,
            Err(err) => {
                eprintln!("Tokenizer error: {:?}", err);
                process::exit(1);
            }
        }
    ).collect();

    for ast in &asts {
        println!("{:?}", ast);
    }

    // intermediate_language.rs
    // compiler.rs
}
