// use indexmap::IndexMap;
use std::io::stdin;

enum SellPattern {
    SingleSell { target_card: usize, sell_num: usize },
    OddSell { sell_num: usize },
    AllSell { sell_num: usize },
}

fn parse_input(
    n: &mut usize,
    q: &mut usize,
    cards_stock: &mut Vec<usize>,
    sell_query: &mut Vec<SellPattern>,
) {
    // read n
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("failed");
    *n = buf.trim_end().parse().unwrap();
    // read card_stock
    buf.clear();
    stdin().read_line(&mut buf).expect("failed");
    let inputs = buf.trim_end().split_whitespace();
    for input in inputs {
        cards_stock.push(input.parse().unwrap());
    }
    // read q
    buf.clear();
    stdin().read_line(&mut buf).expect("failed");
    *q = buf.trim_end().parse().unwrap();
    // read sell_query
    for _i in 0..*q {
        buf.clear();
        stdin().read_line(&mut buf).expect("failed");
        let query_strs: Vec<_> = buf.trim_end().split_ascii_whitespace().collect();
        let input_type: usize = query_strs[0].parse().unwrap();
        match input_type {
            1 => sell_query.push(SellPattern::SingleSell { query_strs[1].parse().unwrap(),  }),
        }
    }
}

fn main() {
    let mut n = 0;
    let mut q = 0;
    let mut cards_stock: Vec<usize> = vec![];
    let mut sell_query: Vec<SellPattern> = vec![];

    parse_input(&mut n, &mut q, &mut cards_stock, &mut sell_query);
}
