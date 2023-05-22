use clap::{Parser, Subcommand};
use rlci::interpreter::Session;
use rlci::parse;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{stdin, BufRead};

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
        Commands::Repl => {
            cmd_repl();
        }
    }
}

fn cmd_parse(input: &str) {
    let res = parse(input);
    println!("{:#?}", res);
}

fn cmd_repl() {
    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let mut session = Session::new();
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(&input).unwrap();
                match parse(&input) {
                    Ok(module) => {
                        let result = session.eval_module(&module);
                        println!("{}", result.repr());
                    }
                    Err(err) => println!("{err}"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
