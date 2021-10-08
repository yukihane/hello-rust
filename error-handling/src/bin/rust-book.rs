// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// にある、Error を Box する方法。ちなみに日本語版
// https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html
// にはまだ無い

// 実践Rustプログラミング入門 p.173
// 独自エラー定義

use std::error::Error;

fn get_int_from_file() -> Result<i32, Box<dyn Error>> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)?;

    let res = num_str.trim().parse::<i32>().map(|t| t * 2)?;

    Ok(res)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
