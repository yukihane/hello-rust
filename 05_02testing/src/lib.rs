//! `adder`クレートはある数値を数値に加える関数を提供する
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

/// この関数は引数に2を加える
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 実行に1時間掛かるコード
    }
}
