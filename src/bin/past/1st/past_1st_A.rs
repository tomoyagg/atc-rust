use std::io::stdin;

fn is_num(input: &String) -> bool {
    let mut ret: bool = true;

    // 入力が長さ3でない場合false
    if input.len() != 3 {
        return false;
    }

    for c in input.chars() {
        ret = ret && c.is_ascii_digit();
    }
    return ret;
}

fn double_num(input: &String) -> i32 {
    return input.parse::<i32>().unwrap() * 2;
}

fn main() {
    let mut buffer: String = String::new();
    stdin().read_line(&mut buffer).expect("failed.");

    buffer = buffer.trim_end().to_string(); // enterの分切り取り // いらなかったら消す

    let flag: bool = is_num(&buffer);
    if flag {
        print!("{}", double_num(&buffer));
    } else {
        print!("error");
    }
}
