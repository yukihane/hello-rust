pub mod wordcounter;

use std::env;
use std::error::Error;
use std::fs::File;

use anyhow::Context;

use crate::wordcounter::count_words;

fn main() -> Result<(), Box<dyn Error>> {
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let mut reader = File::open(&filename).context(format!("unable to opne '{}'", filename))?;

        let wordcount =
            count_words(&mut reader).context(format!("unable to count words in '{}'", filename))?;
        println!("{} {}", wordcount, filename);
    }
    Ok(())
}
