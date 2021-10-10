// 回答その1: https://ja.stackoverflow.com/a/82979/2808

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        // Result::map()は、Ok(x)ならxにクロージャーを適用して得た値yをOkで
        // 包んでOk(y)を返す。Err(e)ならErr(e)をそのまま返す
        .map(|s| s.parse::<i32>().map(|n| n * 2))
        .collect();
    println!("Results: {:?}", numbers);
}
