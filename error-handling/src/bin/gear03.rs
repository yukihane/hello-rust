// 実践Rustプログラミング入門 p.173
// 独自エラー定義

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|e| MyError::Io(e))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e| MyError::Num(e))
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => match e {
            MyError::Io(cause) => println!("{}", cause),
            MyError::Num(cause) => println!("{}", cause),
        },
    }
}
