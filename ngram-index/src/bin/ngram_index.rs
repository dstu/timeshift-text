extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches =
        App::new("ngram_index")
        .version("0.1.0")
        .about("Indexes Google Books TSV data by year")
        .arg(Arg::with_name("dictionary-out")
             .long("dictionary-out")
             .help("Output file for dictionary"))
        .arg(Arg::with_name(""));    
}
