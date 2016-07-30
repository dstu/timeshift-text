/// Just print out each ngram for now.

extern crate ngram_indexer;

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut reader = ngram_indexer::tsv_gz_reader(argv[1].clone()).unwrap();
    for result in reader.decode::<ngram_indexer::GoogleBooksNgramEntry>() {
        match result {
            Ok(entry) => println!("{}", entry.ngram),
            Err(e) => println!("Error: {}", e),
        }
    }
}
