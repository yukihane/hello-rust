// and_then を利用すれば可能だが
// この and_then を省きたい (その2)

use anyhow::{anyhow, Result};

fn parse(s: &str) -> Result<i32> {
    let res = s.parse::<i32>()?;
    Ok(res)
}

fn x2(i: i32) -> Result<i32> {
    if i < 50 {
        Ok(i * 2)
    } else {
        Err(anyhow!("too large"))
    }
}

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| parse(s).and_then(|i| x2(i).and_then(|j| x2(j))))
        .collect();
    println!("Results: {:?}", numbers);
    // Results: [Err(invalid digit found in string), Err(too large), Ok(72)]
}
