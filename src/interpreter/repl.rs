use crate::interpreter::Session;
use crate::parse;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn run_repl() {
    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history("history.txt").is_err() {
        println!("{}", "No previous history.".yellow());
    }
    let mut session = Session::new();
    if let Err(err) = session.load_stdlib() {
        let msg = format!("{:?}", err.context("failed to load stdlib"));
        println!("{}", msg.red());
    }
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(&input).unwrap();
                match parse(&input) {
                    Ok(module) => match session.eval_module(&module) {
                        Ok(result) => println!("{}", result.repr().green()),
                        Err(err) => println!("{}", format!("{:?}", err).red()),
                    },
                    Err(err) => println!("{}", err.to_string().red()),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "CTRL-C".yellow());
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "CTRL-D".yellow());
                break;
            }
            Err(err) => {
                println!("{}", err.to_string().red());
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
