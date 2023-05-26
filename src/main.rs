use clap::{Parser, Subcommand};
use rlci::interpreter::run_repl;
use rlci::parse;

use std::io::{stdin, BufRead};

// Clap is a Rust library for making nice CLI tools.
// It has a few methods for describing the interface you want.
// Here I use the "derive" approach described here:
//
// https://docs.rs/clap/latest/clap/_derive/index.html
//
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a module and print its AST.
    Parse,
    /// Run interactive REPL.
    Repl,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Parse => {
            let mut input = String::new();
            for line in stdin().lock().lines() {
                input.extend(line);
            }
            cmd_parse(&input);
        }
        Commands::Repl => run_repl(),
    }
}

fn cmd_parse(input: &str) {
    let res = parse(input);
    println!("{:#?}", res);
}
