use crate::interpreter::{AutoCompleter, Session};
use crate::parse;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;

pub fn run_repl() {
    let completer = AutoCompleter::new();
    let mut rl: Editor<&AutoCompleter, FileHistory> = Editor::new().unwrap();
    rl.set_helper(Some(&completer));
    if rl.load_history("history.txt").is_err() {
        println!("{}", "No previous history.".yellow());
    }
    let mut session = Session::new(&completer);
    if let Err(err) = session.load_stdlib() {
        let msg = format!("{:?}", err.context("failed to load stdlib"));
        println!("{}", msg.red());
    }
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(&input).unwrap();
                let res = match parse(&input) {
                    Ok(module) => match session.eval_module(&module) {
                        Ok(result) => result.repr().green(),
                        Err(err) => format!("{:?}", err).red(),
                    },
                    Err(err) => err.to_string().red(),
                };
                println!("{}", res);
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
