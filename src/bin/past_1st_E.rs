use std::io::stdin;

enum Log {
    Follow(usize, usize),
    FolowBack(usize),
    FollowFollow(usize),
}

fn parse_input(n: &mut usize, q: &mut usize, inst: &mut Vec<Log>) {
    // recieve N and Q
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed"); // recieve N
    buf = buf.trim_end().to_string();
    let iter: Vec<_> = buf.split_ascii_whitespace().collect();
    *n = iter[0].parse().unwrap();
    *q = iter[1].parse().unwrap();
    // recieve logs
    for _i in 0..*q {
        buf.clear();
        stdin().read_line(&mut buf).expect("failed");
        buf = buf.trim_end().to_string();
        let iter: Vec<_> = buf.split_ascii_whitespace().collect();
        let log_type: usize = iter[0].parse().unwrap();
        match log_type {
            1 => inst.push(Log::Follow(
                iter[1].parse::<usize>().unwrap() - 1,
                iter[2].parse::<usize>().unwrap() - 1,
            )),
            2 => inst.push(Log::FolowBack(iter[1].parse::<usize>().unwrap() - 1)),
            3 => inst.push(Log::FollowFollow(iter[1].parse::<usize>().unwrap() - 1)),
            0 | 4.. => print!("wrong input detected"),
        }
    }
}

fn init_table(table: &mut Vec<Vec<usize>>, size: usize) {
    let mut base_row = vec![];
    for _i in 0..size {
        base_row.push(0);
    }
    for _i in 0..size {
        table.push(base_row.clone());
    }
}
fn follow(following: &usize, followed: &usize, follow_table: &mut Vec<Vec<usize>>) {
    follow_table[*following][*followed] = 1;
}
fn follow_back(user: usize, follow_table: &mut Vec<Vec<usize>>) {
    for target in 0..follow_table.len() {
        if follow_table[target][user] == 1 {
            follow(&user, &target, follow_table);
        }
    }
}
fn follow_follow(user: usize, init_state: &Vec<Vec<usize>>, target_table: &mut Vec<Vec<usize>>) {
    for connector in 0..init_state.len() {
        if init_state[user][connector] == 1 {
            for target in 0..init_state.len() {
                if init_state[connector][target] == 1 {
                    if user != target {
                        follow(&user, &target, target_table);
                    }
                }
            }
        }
    }
}
fn replay_logs(inst: &Vec<Log>, mut table: &mut Vec<Vec<usize>>) {
    for log in inst {
        match log {
            Log::Follow(following, followed) => follow(following, followed, &mut table),
            Log::FolowBack(user) => follow_back(*user, &mut table),
            Log::FollowFollow(user) => follow_follow(*user, &table.clone(), &mut table),
        }
    }
}

fn print_table(table: &Vec<Vec<usize>>) {
    for row in table {
        for cell in row {
            if *cell == 0 {
                print!("N");
            } else {
                print!("Y");
            }
        }
        println!("");
    }
}

fn main() {
    let mut n = 0;
    let mut q = 0;
    let mut inst = vec![];
    parse_input(&mut n, &mut q, &mut inst);

    // フォロー関係テーブル
    let mut follow_table: Vec<Vec<usize>> = vec![];
    init_table(&mut follow_table, n);

    replay_logs(&inst, &mut follow_table);

    print_table(&follow_table);
}
