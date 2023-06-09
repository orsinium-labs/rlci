use crate::interpreter::{Helper, Session};
use crate::parse;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::Editor;

/// Run interactive rustyline-powered REPL.
///
/// "REPL" means "Read, Eval, Print, Loop".
/// It reads input, evaluates it, prints the evaluation result,
/// and repeats the same until the user closes the REPL.
pub fn run_repl() {
    let helper = Helper::new();
    let mut rl: Editor<&Helper, FileHistory> = Editor::new().unwrap();
    rl.set_helper(Some(&helper));
    if rl.load_history("history.txt").is_err() {
        println!("{}", "No previous history.".yellow());
    }
    let mut session = Session::new(Some(&helper));
    if let Err(err) = session.load_stdlib() {
        let msg = format!("{:?}", err.context("failed to load stdlib"));
        println!("{}", msg.red());
    }
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(input) => {
                // We currently do not support multiline expressions in REPL,
                // and a statement is a part of a statement, so let's explicitly
                // allow and skip comments.
                if input.starts_with('#') {
                    continue;
                }
                rl.add_history_entry(&input).unwrap();
                // Parse, eval, and print the input.
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
