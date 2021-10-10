// 回答その2: map_ok を使う https://ja.stackoverflow.com/a/82982/2808

use itertools::Itertools;

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map_ok(|i| i * 2)
        .collect();
    println!("Results: {:?}", numbers);
}
