use std::fs;

use rlci::parse;

fn main() {
    let content = fs::read_to_string("src/stdlib/bool.txt").unwrap();
    let res = parse(&content);
    println!("{:#?}", res);
}
