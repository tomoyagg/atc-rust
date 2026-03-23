use std::io::stdin;
extern crate regex;
use regex::Regex;

// ソート処理の実装が必要

fn split_phrase(buf: &String) -> Vec<&str> {
    let re = Regex::new(r"[A-Z][a-z]*[A-Z]").unwrap();

    let mut v = vec![];
    for caps in re.captures_iter(&buf) {
        v.push(caps.get(0).unwrap().as_str());
    }
    return v;
}

fn main() {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed");

    let mut phrases = split_phrase(&buf);

    phrases.sort(); // wrong

    for str in phrases {
        print!("{}", str);
    }
}
