extern crate phrases;

use phrases::english::{farewells, greetings};

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());
    //
    //    println!(
    //        "Hello in Japanese: {}",
    //        phrases::japanese::greetings::hello()
    //    );
    //    println!(
    //        "Goodbye in Japanese: {}",
    //        phrases::japanese::farewells::goodbye()
    //    );
}
