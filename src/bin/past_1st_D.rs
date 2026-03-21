use std::io::stdin;

fn parse_input() -> Vec<usize> {
    let mut ret: Vec<usize> = vec![];

    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed");
    buf = buf.trim_end().to_string();
    if buf.len() == 0 {
        return ret;
    }
    let input_size = buf.parse::<usize>().unwrap();
    if input_size != 0 {
        for _i in 0..input_size {
            buf.clear();
            stdin().read_line(&mut buf).expect("failed");
            buf = buf.trim_end().to_string();
            ret.push(buf.parse::<usize>().unwrap());
        }
    }

    return ret;
}

fn out_diff(input: &Vec<usize>) -> Vec<usize> {
    let mut ret: Vec<usize> = vec![];
    for i in 0..input.len() - 1 {
        ret.push(input[i + 1] - input[i]);
    }
    return ret;
}

fn check_iso(
    input: &Vec<usize>,
    diff: &Vec<usize>,
    lack: &mut usize,
    zero_increase: &mut usize,
) -> bool {
    let mut ret = false;
    let mut num = 0;
    while num < diff.len() {
        if diff[num] == 1 {
            num = num + 1;
        } else {
            break;
        }
    }
    if num == diff.len() {
        ret = true;
    } else {
        while diff[*zero_increase] != 0 {
            *zero_increase = *zero_increase + 1;
        }
        while input.contains(lack){
            *lack = *lack + 1;
        }
    }
    return ret;
}

fn main() {
    let mut input = parse_input();
    if input.len() < 1 {
        println!("Correct");
    } else {
        input.sort();
        let diff = out_diff(&input);
        let mut lack: usize = 1;
        let mut zero_increase: usize = 0;
        let iso = check_iso(&input, &diff, &mut lack, &mut zero_increase);
        if iso {
            println!("Correct");
        } else {
            println!("{} {}", input[zero_increase], lack);
        }
    }
}
