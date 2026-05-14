mod cli;
mod ignore;
mod render;
mod tree;

use clap::Parser;
use std::io::IsTerminal;

use crate::cli::{Cli, OutputFormat};
use crate::tree::TreeOptions;

fn main() {
    let cli = Cli::parse();

    let options = TreeOptions {
        max_depth: cli.depth,
        all: cli.all,
        dirs_only: cli.dirs_only,
        no_gitignore: cli.no_gitignore,
        ignore_patterns: cli.ignore_patterns,
    };

    let built = match tree::build_tree(&cli.path, &options) {
        Ok(tree) => tree,
        Err(error) => {
            eprintln!("pytree: {error}");
            std::process::exit(2);
        }
    };

    let rendered = match cli.format {
        OutputFormat::Tree => render::render_tree(
            &built.root,
            std::io::stdout().is_terminal(),
            if cli.ascii {
                render::TreeStyle::Ascii
            } else {
                render::TreeStyle::Unicode
            },
        ),
        OutputFormat::Json => match render::render_json(&built.root) {
            Ok(json) => json,
            Err(error) => {
                eprintln!("pytree: failed to render JSON: {error}");
                std::process::exit(2);
            }
        },
    };

    print!("{rendered}");
    if !rendered.ends_with('\n') {
        println!();
    }

    for error in built.errors {
        eprintln!("pytree: skipped entry: {error}");
    }
}
