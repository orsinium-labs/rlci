use std::cell::RefCell;
use std::collections::HashSet;

use rustyline::hint::Hint;
use rustyline::Context;

pub struct LangHinter {
    hints: RefCell<HashSet<CommandHint>>,
}

impl LangHinter {
    pub fn new() -> Self {
        Self {
            hints: HashSet::new().into(),
        }
    }

    pub fn add(&self, text: &str) {
        let h = CommandHint(text.to_string());
        self.hints.borrow_mut().insert(h);
    }
}

impl rustyline::validate::Validator for LangHinter {}
impl rustyline::highlight::Highlighter for LangHinter {}
impl rustyline::Helper for LangHinter {}

impl rustyline::completion::Completer for LangHinter {
    type Candidate = CommandHint;
}

impl rustyline::hint::Hinter for LangHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints.borrow().iter().find_map(|hint| {
            // expect hint after word complete, like redis cli, add condition:
            // line.ends_with(" ")
            if hint.display().starts_with(line) {
                Some(hint.suffix(pos))
            } else {
                None
            }
        })
    }
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct CommandHint(String);

impl CommandHint {
    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint(self.0[strip_chars..].to_string())
    }
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.0
    }

    fn completion(&self) -> Option<&str> {
        Some(&self.0)
    }
}

impl rustyline::completion::Candidate for CommandHint {
    fn display(&self) -> &str {
        &self.0
    }

    fn replacement(&self) -> &str {
        &self.0
    }
}
