use std::collections::HashMap;
use std::io::stdin;
extern crate regex;
use regex::Regex;

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

    let phrases = split_phrase(&buf);

    let mut upper_phrases: Vec<_> = vec![];
    for str in phrases.clone() {
        upper_phrases.push(str.to_ascii_uppercase());
    }
    upper_phrases.sort();

    let mut to_upper_map: HashMap<_, _> = HashMap::new();
    for str in phrases {
        to_upper_map.insert(str.to_ascii_uppercase(), str);
    }

    for str in upper_phrases {
        print!("{}", to_upper_map[&str]);
    }
}