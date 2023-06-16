# RLCI

🦀 Overly-documented Rust-powered Lambda Calculus Interpreter.

## 😎 Features

+ 📚 **Overly-documented**. There are comments and docstrings for everything, almost every line. Our top priority is to provide you with a good example of building your own programming language.
+ 🐦 **Simple**. We kept only essentials and ditched anything unneded. Comments? Strings? System calls? Who needs it!
+ ⚙️ **Working**. Everything possible in lambda calculus is also possible with RLCI.
+ 🏃 **REPL**. We have an interactive input with autocomplete and a bit of syntax highlighting.
+ ⚠️ **Friendly error messages**. And we even got tracebacks!
+ 🦀 **Rust**. Rust is a good choice for writing a programming language, thanks to binary distributions and great performance.

## 🧠 Learn Lambda Calculus

The main idea of [Lambda calculus](https://en.wikipedia.org/wiki/Lambda_calculus) is pretty simple. What if you have a programming language where you can only define a single-argument function and call other functions? That's it. No integers, no strings, no loops, no conditions. Just you and functions. And turns out, you can do quite a few things. In fact, anything that any other programming language can do.

The best way to learn lambda calculus is the [Lambda Calculus from the Ground Up](https://www.youtube.com/watch?v=pkCLMl0e_0k) workshop by David Beazley. I recommend you to open Python interpreter, follow along, and pause and try to think for yourself when the speaker asks a question. It's a great exercise and this is the most mind-boggling thing I ever learned.

When you go through the workshop, take a look at the [python-lambda-calculus](https://github.com/orsinium-labs/python-lambda-calculus) project. There, I've implemented eveything covered on the workshop and a few more things, like filter, map, reduce, and signed integers.

## 🤔 Motivation

I had several attempts to write a programming language before. And each time I ended up with [scope creep](https://en.wikipedia.org/wiki/Scope_creep) and a spaghetti of features breaking each other and at the end nothing works. This time, I decided to approach things differently and make the smallest possible working programming language.

If you want a [Turing complete](https://en.wikipedia.org/wiki/Turing_completeness) programming language, your best options are either [Turing machine](https://en.wikipedia.org/wiki/Turing_machine) or Lambda calculus. The thing about Turing machine, though, is that it's quite hard to program on it anything meaningful (see [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)). But Lambda calculus is different. Lambda calculus is a real functional language and has almost everything you have in LISP or Clojure. So, here we are.

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

## Questions and Answers

**Q: Should I use it on the production?**

A: No. Modern computers are designed after Turing machine rather than Lambda calculus. So, it will be slow. Very slow. Still better than [DrRacket](https://docs.racket-lang.org/drracket/), though.

**Q: Why there are no type annotations?**

A: Because there is only one type: a function.

**Q: Why does it exist?**

Because I can and I needed a break from tossing JSONs from one API to another.

**Q: When is the next release?**

A: Lambda Calculus became feature complete in 1930s. I'll let you know if there is anything new.

**Q: Why is it on Rust?**

Because writing things on Python or Go is too easy. Writing on Rust is like solving riddles, and I love riddles.

**Q: Should I star the project?**

A: Yes, it will make me look superior to the other people the the office.
