use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Input file (defaults to stdin if not provided)
    #[arg(long = "source", alias = "src", short = 's')]
    pub source: Option<PathBuf>,

    /// Output file (defaults to stdout if not provided)
    #[arg(long = "target", alias = "output", short = 't')]
    pub target: Option<PathBuf>,

    /// Enable debug mode
    #[arg(long, short)]
    pub debug: bool,

    #[arg(long, value_enum, default_value_t = Mode::Normal)]
    pub mode: Mode
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Mode {
    Normal,
    Minimized,
    Obfuscated,
    Extended,
}