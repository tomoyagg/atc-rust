use std::io::stdin;

fn parse_input(a: &mut usize, b: &mut usize, c: &mut usize, s: &mut String) {
    let mut buf: String = String::new();
    // read a
    stdin().read_line(&mut buf).expect("failed");
    *a = buf.trim_end().parse().unwrap();
    buf.clear();
    // read b,c
    stdin().read_line(&mut buf).expect("failed");
    let nums: Vec<&str> = buf.trim_end().split_ascii_whitespace().collect();
    *b = nums[0].parse().unwrap();
    *c = nums[1].parse().unwrap();
    buf.clear();
    // read s
    stdin().read_line(&mut buf).expect("failed");
    *s = buf.trim_end().to_string();
}

fn main() {
    let mut a: usize = 0;
    let mut b: usize = 0;
    let mut c: usize = 0;
    let mut s: String = String::new();

    parse_input(&mut a, &mut b, &mut c, &mut s);

    print!("{} ", a + b + c);

    print!("{}", s);
}
