// map_ok は Result を返すものを利用すると扱いづらい
// Result がネストするので

use itertools::Itertools;

fn x2(i: i32) -> Result<i32, &'static str> {
    if i < 50 {
        Ok(i * 2)
    } else {
        Err("too large")
    }
}

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map_ok(|i| x2(i))
        .collect();
    println!("Results: {:?}", numbers);
    // Results: [Err(ParseIntError { kind: InvalidDigit }), Ok(Err("too large")), Ok(Ok(36))]
}
