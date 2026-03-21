use std::io::{stdin};

fn parse_input() -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];

    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed");
    buf = buf.trim_end().to_string();

    let vec: Vec<&str> = buf.split(' ').collect();

    for s in vec {
        ret.push(s.parse().unwrap());
    }

    return ret;
}

fn main() {
    let mut input = parse_input();
    input.sort();
    println!("{}", input[input.len() - 3]);
}
