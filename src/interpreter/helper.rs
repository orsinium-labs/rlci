use colored::Colorize;
use rustyline::hint::Hint;
use rustyline::Context;
use std::borrow::Cow;
use std::cell::RefCell;

// Helper is a struct that provides autocomplete and syntax highlighting for rustyline.
//
// The name "Helper" is bad and meaningless but that's how rostyline calls it.
// I have no idea why the mix together into one class all the logic of autocomplete,
// syntax highlighting, validation, and everything else.
pub struct Helper {
    hints: RefCell<Vec<CommandHint>>,
}

impl Helper {
    pub fn new() -> Self {
        Self {
            hints: Vec::new().into(),
        }
    }

    /// Add a new name to be used in autocomplete.
    ///
    /// This method is called by the Session for each new assignment.
    /// We do not bother with autocomplete for local variables
    /// because the lazy convention is to use single letters for them anyway.
    pub fn add(&self, text: &str) {
        let h = CommandHint(text.to_string());
        self.hints.borrow_mut().push(h);
    }
}

// These traits are required to be implemented for the type
// to be able to pass it into `Editor.set_helper`.
//
// The official rustyline example uses `derive` with these traits
// but somehow it doesn't work for me.
impl rustyline::validate::Validator for Helper {}
impl rustyline::Helper for Helper {}

// Provides a very basic syntax highlighting for all user input in the REPL.
impl rustyline::highlight::Highlighter for Helper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let _ = pos;

        // Create colored versions of the known tokens.
        let lambda = "λ".blue().to_string();
        let equal = "=".blue().to_string();
        let left_br = "(".magenta().to_string();
        let right_br = ")".magenta().to_string();
        let hash = "#".cyan().to_string();

        // Just in case, remove all previous syntax highlighting.
        let line = line.clear();

        // Replace known tokens with colored versions.
        //
        // For a real production-quality language, you should instead
        // actually try to `parse` the input, iterate over the resulting tokens,
        // and highlight them in the input text based on their positions.
        // But for now, let's keep it simple.
        let line = line.replace('λ', &lambda);
        let line = line.replace('\\', &lambda);
        let line = line.replace('=', &equal);
        let line = line.replace('(', &left_br);
        let line = line.replace(')', &right_br);
        let line = line.replace('#', &hash);

        Cow::Owned(line)
    }

    // We make this method to always return true,
    // so that syntax highlighting kicks in every time the user presses a button.
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        let _ = (line, pos);
        true
    }
}

// Implement a very basic autocomplete.
impl rustyline::completion::Completer for Helper {
    type Candidate = CommandHint;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut res: Vec<CommandHint> = Vec::new();
        // Autocomplete only if the cursor is at the very end of the input string.
        if line.is_empty() || pos < line.len() {
            return Ok((pos, res));
        }

        // Take the last word and try to find all known names starting with it.
        let (_, word) = line.rsplit_once(' ').unwrap_or(("", line));
        for hint in self.hints.borrow().iter() {
            if hint.display().starts_with(word) {
                res.push(hint.suffix(word.len()))
            }
        }
        Ok((pos, res))
    }
}

// Everything below is pretty much copy-pasted from an example in the rustyline repo.
//
// https://github.com/kkawakam/rustyline/blob/master/examples/diy_hints.rs
impl rustyline::hint::Hinter for Helper {
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
