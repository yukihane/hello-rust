fn main() {
    println!("1+1=??");
    println!("??の値を入力してください");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().parse::<i32>().unwrap();

    dbg!(input);
}
