use clap::{Parser, Subcommand};
use colored::Colorize;
use rlci::interpreter::run_repl;
use rlci::interpreter::{AutoCompleter, Session};
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
    /// Eval a module and print the last expression result.
    Eval,
    /// Run interactive REPL.
    Repl,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Parse => cmd_parse(&read_stdin()),
        Commands::Eval => cmd_eval(&read_stdin()),
        Commands::Repl => run_repl(),
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    for line in stdin().lock().lines() {
        input.extend(line);
        input.push('\n');
    }
    input
}

fn cmd_parse(input: &str) -> ! {
    let (code, msg) = match parse(input) {
        Ok(module) => (0, format!("{:#?}", module).green()),
        Err(err) => (3, err.to_string().red()),
    };
    println!("{}", msg);
    std::process::exit(code);
}

fn cmd_eval(input: &str) -> ! {
    let completer = AutoCompleter::new();
    let mut session = Session::new(&completer);
    if let Err(err) = session.load_stdlib() {
        let msg = format!("{:?}", err.context("failed to load stdlib"));
        println!("{}", msg.red());
        std::process::exit(1);
    }
    let (code, msg) = match parse(input) {
        Ok(module) => match session.eval_module(&module) {
            Ok(result) => (0, result.repr().green()),
            Err(err) => (2, format!("{:?}", err).red()),
        },
        Err(err) => (3, err.to_string().red()),
    };
    println!("{}", msg);
    std::process::exit(code);
}
