use std::io::{stdin};

fn parse_input() -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed");
    buf = buf.trim().to_string();
    let input_count: i32 = buf.parse().unwrap();

    for _i in 0..input_count {
        buf.clear();
        stdin().read_line(&mut buf).expect("failed");
        buf = buf.trim().to_string();
        ret.push(buf.parse().unwrap());
    }
    return ret;
}

fn diff(input: &Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];

    for i in 0..input.len() - 1 {
        ret.push(input[i + 1] - input[i]);
    }
    return ret;
}

fn out(input: &Vec<i32>) {
    for num in input {
        if *num > 0 {
            println!("up {}", num);
        } else if *num == 0 {
            println!("stay");
        } else {
            println!("down {}", num * (-1));
        }
    }
}

fn main() {
    let input: Vec<i32> = parse_input();
    let diff: Vec<i32> = diff(&input);
    out(&diff);
}
