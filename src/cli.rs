use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
#[command(
    name = "pytree",
    version,
    about = "Print a clean tree view of a Python project"
)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long, value_name = "N")]
    pub depth: Option<usize>,

    #[arg(long)]
    pub all: bool,

    #[arg(long)]
    pub dirs_only: bool,

    #[arg(long)]
    pub no_gitignore: bool,

    #[arg(long)]
    pub ascii: bool,

    #[arg(long = "ignore", value_name = "PATTERN")]
    pub ignore_patterns: Vec<String>,

    #[arg(long, value_enum, default_value_t = OutputFormat::Tree)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Tree,
    Json,
}
