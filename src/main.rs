use nom::error::VerboseError;
use rlci::*;
use std::fs;

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
    /// Parse a file and print its AST
    ParseFile {
        path: String,
    },
    ParseExpr {
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::ParseFile { path } => {
            cmd_parse_file(path);
        }
        Commands::ParseExpr { input } => {
            cmd_parse_expr(input);
        }
    }
}

fn cmd_parse_file(path: &String) {
    let input = fs::read_to_string(path).unwrap();
    let res = parse_module::<VerboseError<&str>>(&input);

    if let Err(err) = res {
        if let nom::Err::Error(e) = err {
            println!("{:#?}", e);
            let msg = nom::error::convert_error(&input[..], e);
            println!("{}", msg);
        } else {
            println!("{:#?}", err);
        }
    } else {
        println!("{:#?}", res);
    }
}

fn cmd_parse_expr(input: &str) {
    let res = parse_expr::<VerboseError<&str>>(input);
    println!("{:#?}", res);
}
