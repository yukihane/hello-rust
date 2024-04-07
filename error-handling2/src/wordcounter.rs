use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;

pub fn count_words<R: Read>(input: &mut R) -> Result<u32, Box<dyn Error>> {
    let reader = BufReader::new(input);
    let mut wordcount = 0;
    for line in reader.lines() {
        for _word in line?.split_whitespace() {
            wordcount += 1;
        }
    }
    Ok(wordcount)
}
