// 実践Rustプログラミング入門 p.176
// anyhow利用

use anyhow::Result;

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)?;

    let num = num_str.trim().parse::<i32>().map(|t| t * 2)?;

    Ok(num)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
