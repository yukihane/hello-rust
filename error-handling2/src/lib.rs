pub mod wordcounter;

use std::env;
use std::error::Error;
use std::fs::File;

use crate::wordcounter::count_words;

pub fn run() -> Result<(), Box<dyn Error>> {
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let mut reader = File::open(&filename)?;

        let wordcount = count_words(&mut reader)?;
        println!("{} {}", wordcount, filename);
    }
    Ok(())
}
