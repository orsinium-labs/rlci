use std::io::{stdin, BufRead};

use rlci::parse;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a module and print its AST
    ParseExpr,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::ParseExpr => {
            let mut input = String::new();
            for line in stdin().lock().lines() {
                input.extend(line);
            }
            cmd_parse_expr(&input);
        }
    }
}

fn cmd_parse_expr(input: &str) {
    let res = parse(input);
    println!("{:#?}", res);
}
