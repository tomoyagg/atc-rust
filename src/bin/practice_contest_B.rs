use std::io::{self, Write, stdin};

fn parse_input(n: &mut usize, q: &mut usize) {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed");
    let nums: Vec<_> = buf.trim_end().split_ascii_whitespace().collect();
    *n = nums[0].parse().unwrap();
    *q = nums[1].parse().unwrap();
}

fn swap(alphabets: &mut Vec<char>, i: usize, j: usize) {
    let temp: char = alphabets[i];
    alphabets[i] = alphabets[j];
    alphabets[j] = temp;
}

fn query(alphabets: &mut Vec<char>, i: usize, j: usize) -> bool {
    // throw a query
    print!("? {0} {1}", alphabets[i], alphabets[j]);
    io::stdout().flush().unwrap();
    // recieve answer
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed");
    let mut answer = buf.trim_end();
    if answer == "<" {
        return true;
    } else if answer == ">" {
        return false;
    } else {
        print!("error: query answer mismatch.");
        return false;
    }
}

fn compare(alphabets: &mut Vec<char>, q: usize) {

    // PLAN 
    // 大小関係不明未定判定を行う→計算量が大きい(課題、どうするか考え中...)
    // 不明とわかる場合、判定をquery投げる
}

fn main() {
    let mut n: usize = 0;
    let mut q: usize = 0;
    parse_input(&mut n, &mut q);

    if n == 5 {
        let mut alphabets: Vec<char> = "ABCDE".chars().collect();
        compare(&mut alphabets, q);
    }
    if n == 26 {
        let mut alphabets: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        compare(&mut alphabets, q);
    }
}
