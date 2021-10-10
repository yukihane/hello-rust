// https://ja.stackoverflow.com/q/82978/2808 のコード

use std::num::ParseIntError;

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map::<Result<_, ParseIntError>, _>(|r| Ok(r? * 2))
        .collect();
    println!("Results: {:?}", numbers);
}
