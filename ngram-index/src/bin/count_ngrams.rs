extern crate clap;
extern crate csv;
extern crate ngram_indexer;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::path;

use clap::{App, Arg};

#[derive(Debug, Eq, PartialEq, Hash)]
struct CountKey {
    year: u16,
    ngram: Vec<u32>,
}

#[derive(Debug, Default)]
struct CountValue {
    term_frequency: u32,
    document_frequency: u32,
}

fn main() {
    let arg_matches =
        App::new("count_ngrams")
        .version("0.1.0")
        .about("Builds tables that count n-gram occurrences per year from Google Books TSV data")
        .arg(Arg::with_name("out")
             .long("out")
             .short("o")
             .takes_value(true)
             .required(true)
             .help("Output file for serialized counts data"))
        .arg(Arg::with_name("dictionary")
             .long("dictionary")
             .short("d")
             .takes_value(true)
             .required(true)
             .help("Dictionary file created with build_dictionary that maps unigrams to IDs"))
        .arg(Arg::with_name("drop_pos_ngrams")
             .long("drop_pos_ngrams")
             .help("Drop ngrams a POS element present"))
        .arg(Arg::with_name("input")
             .help("File(s) to read from")
             .required(true)
             .index(1)
             .multiple(true))
        .get_matches();

    let dictionary_file = arg_matches.value_of("dictionary")
        .map(|s| {
            let p = path::Path::new(s);
            if p.is_dir() {
                panic!("Dictionary path cannot be a directory")
            } else {
                p
            }
        }).unwrap();
    let output_file = arg_matches.value_of("out")
        .map(|s| {
            let p = path::Path::new(s);
            if p.is_dir() {
                panic!("Output path cannot be a directory")
            } else {
                p
            }
        }).unwrap();
    let drop_pos_ngrams = arg_matches.occurrences_of("drop_pos_ngrams") > 0;
    let input_paths: Vec<String> =
        arg_matches.values_of("input").unwrap().map(|x| x.to_string()).collect();
    
    // TODO(dstu): load dictionary from dictionary_file.

    // We keep counts in a HashMap and merge them when we find duplicate n-grams
    // because unknown word subtitution may give us multiple identical ngrams.
    let mut counts: HashMap<CountKey, CountValue> = HashMap::new();
    let mut years_seen: HashSet<u16> = HashSet::new();
    for input_path in input_paths.into_iter() {
        let mut reader = ngram_indexer::tsv_gz_reader(input_path.clone()).unwrap();
        let mut entries = reader.decode::<ngram_indexer::GoogleBooksNgramEntry>()
            .enumerate()
            .map(|(i, result)|
                 match result {
                     Ok(entry) => entry,
                     Err(e) => panic!("error processing entry at {}:{} ({:?})", input_path, i, e),
                 });
        'each_entry: for entry in entries {
            let tokens: Vec<&str> = entry.ngram.split(' ').collect();
            let mut token_ids = Vec::with_capacity(tokens.len());
            for token in &tokens {
                if drop_pos_ngrams && ngram_indexer::token_is_pos_tag(token) {
                    continue 'each_entry
                }

                // TODO(dstu): look up token in dictionary.
                // let token_id = ...;
                // token_ids.push(token_id);
            }
            years_seen.insert(entry.year);
            let key = CountKey {
                year: entry.year,
                ngram: token_ids,
            };
            match counts.entry(key) {
                Entry::Occupied(mut e) => {
                    let mut extant = e.get_mut();
                    extant.term_frequency += entry.term_frequency;
                    extant.document_frequency += entry.document_frequency;
                },
                Entry::Vacant(e) => {
                    e.insert(CountValue {
                        term_frequency: entry.term_frequency,
                        document_frequency: entry.document_frequency,
                    });
                },
            }
        }
    }

    // TODO(dstu): push counts into a serializable structure and write to disk.
}
