// rust-book.rs の方式で独自定義エラーをハンドルする
// https://doc.rust-jp.rs/rust-by-example-ja/error/multiple_error_types/boxing_errors.html
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
// にやり方が書いてあった

use std::error;
use std::fmt;

#[derive(Debug)]
struct MyError;

impl error::Error for MyError {
    fn description(&self) -> &str {
        "custom error description"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        None
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "custom error displayed")
    }
}

fn validate(value: i32) -> Result<(), MyError> {
    if value < 10 {
        Ok(())
    } else {
        Err(MyError)
    }
}

fn get_int_from_file() -> Result<i32, Box<dyn error::Error>> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)?;

    let res = num_str.trim().parse::<i32>().map(|t| t * 2)?;

    validate(res)?;

    Ok(res)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
