// https://ja.stackoverflow.com/q/82978/2808 のコード

fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .map_and_then(|i /* Ok のときだけ受け取る */| Ok(i * 2))
        .collect();
    println!("Results: {:?}", numbers);
}
