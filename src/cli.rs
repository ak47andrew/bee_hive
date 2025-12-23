use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The path to the file to read
    pub path: std::path::PathBuf,
    /// debug mode
    #[clap(long, short, action)]
    pub debug: bool,
}
