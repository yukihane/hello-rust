#![allow(dead_code)]

#[derive(Debug)]
enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum Person {
    Skinny = 1,
    Fat,
    // タプルや構造体を含んでいるとCライクenumとしてコンパイルできない
    // (コンパイルエラーになる)
    // また, Cライクenumではない場合, as i32 もできない
//    Height(i32),
//    Info { name: String, height: i32 },
}

pub fn main() {
    println!("zero is {:?}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    println!("{}", Person::Fat as i32);
}
