mod translator;
mod cli;

use std::process;
use clap::Parser;
use crate::cli::Cli;
use crate::translator::cleanup::{post_processing, split_statements};
use crate::translator::compiler::codegen;
use crate::translator::intermediate_language::{evaluate, IR};
use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{Expr, tokenize};
use std::io::{self, Read, Write};

fn main() {
    let args = Cli::parse();

    let mut input: Box<dyn Read> = match args.source {
        Some(path) => Box::new(std::fs::File::open(path).unwrap()),
        None => Box::new(io::stdin()),
    };

    let mut output: Box<dyn Write> = match args.target {
        Some(path) => Box::new(std::fs::File::create(path).unwrap()),
        None => Box::new(io::stdout()),
    };


    let mut content = String::new();
    input.read_to_string(&mut content).unwrap();

    let lines = split_statements(&content)
        .iter()
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    if args.debug {
        println!("Statements:\n{:#?}\n", lines);
    }

    let asts: Vec<Expr> = lines
        .iter()
        .map(|x| tokenize(&x))
        .map(
            |x| match x {
                Ok(expr) => expr,
                Err(err) => {
                    eprintln!("Tokenizer error: {}", err);
                    process::exit(1);
                }
            }
    ).collect();

    if args.debug {
        println!("Tokens:\n{:#?}\n", asts);
    }

    let mut memory_manager = MemoryManager::new();
    #[allow(nonstandard_style)]
    let mut IRs: Vec<IR> = Vec::new();
    for ast in asts {
        match evaluate(&ast, &mut memory_manager) {
            Ok(ir) => {
                IRs.extend(ir);
            }
            Err(err) => {
                eprintln!("IR generator error: {}", err);
                process::exit(1);
            }
        }

    }

    if args.debug {
        println!("IRs:\n{:#?}\n", IRs);
    }

    let code = codegen(IRs);

    let post = post_processing(&code, args.mode);

    output.write_all(&*post).unwrap();
}
