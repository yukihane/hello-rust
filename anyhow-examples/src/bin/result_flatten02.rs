// こういうことがやりたい

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
        .map(parse)
        .map(x2)
        .map(x2)
        .collect();
    println!("Results: {:?}", numbers);
}
