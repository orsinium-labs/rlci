use std::fs;

use rlci::parse;

fn main() {
    let input = fs::read_to_string("src/stdlib/bool.txt").unwrap();
    let res = parse(&input);

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
