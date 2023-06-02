use colored::Colorize;
use rustyline::hint::Hint;
use rustyline::Context;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;

pub struct AutoCompleter {
    hints: RefCell<HashSet<CommandHint>>,
}

impl AutoCompleter {
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

impl rustyline::validate::Validator for AutoCompleter {}
impl rustyline::Helper for AutoCompleter {}

impl rustyline::highlight::Highlighter for AutoCompleter {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let _ = pos;

        // create colored versions of the known tokens
        let lambda = "λ".blue().to_string();
        let equal = "=".blue().to_string();
        let left_br = "(".magenta().to_string();
        let right_br = ")".magenta().to_string();
        let hash = "#".cyan().to_string();

        // replace known tokens with colored versions
        let line = line.clear();
        let line = line.replace('λ', &lambda);
        let line = line.replace('\\', &lambda);
        let line = line.replace('=', &equal);
        let line = line.replace('(', &left_br);
        let line = line.replace(')', &right_br);
        let line = line.replace('#', &hash);

        Cow::Owned(line)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        let _ = (line, pos);
        true
    }
}

impl rustyline::completion::Completer for AutoCompleter {
    type Candidate = CommandHint;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut res: Vec<CommandHint> = Vec::new();
        if line.is_empty() || pos < line.len() {
            return Ok((pos, res));
        }

        let (_, word) = line.rsplit_once(' ').unwrap_or(("", line));
        for hint in self.hints.borrow().iter() {
            if hint.display().starts_with(word) {
                res.push(hint.suffix(word.len()))
            }
        }
        Ok((pos, res))
    }
}

impl rustyline::hint::Hinter for AutoCompleter {
    type Hint = CommandHint;
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
