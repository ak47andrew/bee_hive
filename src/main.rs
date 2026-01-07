mod translator;
mod cli;

use std::process;
use clap::Parser;
use crate::cli::Cli;
use crate::translator::compiler::codegen;
use crate::translator::intermediate_language::{evaluate, IR};
use crate::translator::memory_manager::MemoryManager;
use crate::translator::tokenizer::{Expr, tokenize};

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path).unwrap();

    // cleanup.rs
    let lines = translator::cleanup::split_statements(&content);

    // tokenizer.rs

    let asts: Vec<Expr> = lines
        .iter()
        .map(|x| x.strip_prefix('\n').unwrap_or(x.as_str()).to_string())
        .filter(|x| !x.is_empty())
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

    // intermediate_language.rs
    let mut memory_manager = MemoryManager::new();
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

    let code = codegen(IRs);

    println!("{}", code);
}
