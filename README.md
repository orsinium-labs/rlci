# RLCI

🦀 Overly-documented Rust-powered Lambda Calculus Interpreter.

## 😎 Features

+ **Overly-documented**. There are comments and docstrings for everything, almost every line. Our top priority is to provide you with a good example of building your own programming language.
+ **Simple**. We kept only essentials and ditched anything unneded. Comments? Strings? System calls? Who needs it!
+ **Working**. Everything possible in lambda calculus is also possible with RLCI.
+ **REPL**. We have an interactive input with autocomplete and a bit of syntax highlighting.
+ **Friendly error messages**. And we even got tracebacks!

## 🧠 Learn Lambda Calculus

...

## 🤔 Motivation

...

## 📝 Syntax

...

## 📦 Installation

If you have cargo:

```bash
cargo install rlci
```

If you don't, go into [releases](https://github.com/orsinium/rlci/releases) and grab a binary for your system.

## 🛠️ Usage

...

## ⚙️ Dependencies

+ [pest](https://github.com/pest-parser/pest) is for parsing the languagew grammar into AST.
+ [rustyline](https://github.com/kkawakam/rustyline) is for making a friendly REPL.
+ [clap](https://github.com/clap-rs/clap) is for making a friendly CLI.
+ [anyhow](https://github.com/dtolnay/anyhow) is for easy error handling and nice error messages.
+ [include_dir](https://github.com/Michael-F-Bryan/include_dir) is for including stdlib into the binary.
+ [colored](https://github.com/mackwic/colored) is for colorizing terminal output. Errors should be red!
