use std::io::{self, BufRead};

pub fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let l = lines.next().unwrap().unwrap();
    let line = l.chars();

    for (i, val) in line.enumerate() {
        if i % 2 == 0 {
            print!("{}", val);
        }
    }
    println!("");
}
