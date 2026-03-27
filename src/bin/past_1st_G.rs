// use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use std::io::stdin;

#[derive(Hash, Eq, PartialEq)]
struct Base {
    row: usize,
    column: usize,
}

fn parse_input(n: &mut usize, map: &mut FxHashMap<Base, isize>) {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).expect("failed");
    *n = buf.trim_end().parse().unwrap();
    for _i in 0..*n {
        buf.clear();
        stdin().read_line(&mut buf).expect("failed");
        let iter = buf.trim_end().split_whitespace();
        let mut j = _i + 1;
        for num_str in iter {
            map.insert(Base { row: _i, column: j }, num_str.parse().unwrap());
            j += 1;
        }
    }
}

fn calc_max_points(){
    
}

fn main() {
    let mut n: usize = 0;
    let mut relation_map: FxHashMap<Base, isize> = FxHashMap::default();
    parse_input(&mut n, &mut relation_map);

    let mut max_points : Vec<isize> = vec![];

    
}
