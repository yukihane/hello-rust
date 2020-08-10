use std::io::{self, Read};

pub fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer).unwrap();

    let res = buffer
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .sum::<i32>()
        % 10;
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
