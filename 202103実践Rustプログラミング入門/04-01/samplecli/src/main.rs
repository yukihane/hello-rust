use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome RPN calculator"
)]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,
    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        // ファイルを指定しなかった場合
        println!("No file is specified");
    }
}

fn run(reader: BufReader<File>, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
