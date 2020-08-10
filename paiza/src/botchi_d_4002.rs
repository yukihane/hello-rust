use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    lines.next();
    lines
        .filter_map(|x| {
            let y = x.unwrap();
            let line = y.split_whitespace().collect::<Vec<_>>();
            let e: i32 = line.get(1).unwrap().parse().unwrap();
            if e != 3 {
                None
            } else {
                let s = line.get(0).unwrap();
                Some(s.to_string())
            }
        })
        .for_each(|x| println!("{}", x));
}
