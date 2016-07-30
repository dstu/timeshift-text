/// Just print out each ngram for now.

extern crate clap;
extern crate csv;
extern crate itertools;
extern crate ngram_indexer;

use std::path;

use clap::{App, Arg};
use itertools::Itertools;
// use ngram_indexer::ngram::dictionary;

fn main() {
    let arg_matches =
        App::new("build_dictionary")
        .version("0.1.0")
        .about("Builds a dictionary of n-grams from Google Books TSV data")
        .arg(Arg::with_name("out")
             .long("out")
             .short("o")
             .takes_value(true)
             .required(true)
             .help("Output file for serialized capnp dictionary"))
        .arg(Arg::with_name("tf_threshold")
             .long("tf-threshold")
             .help("Minimum term frequency (summed across all years) that ngram must have or else be lumped into unknown token category"))
        .arg(Arg::with_name("df_threshold")
             .long("df-threshold")
             .help("Minimum document frequency (summed across all years) that ngram must have or else be lumped into unknown token category"))
        .arg(Arg::with_name("unknown_token")
             .long("unk-token")
             .takes_value(true)
             .help("Unknown-word token to substitute for tokens that have been dropped"))
        .arg(Arg::with_name("drop_token_pos")
             .long("drop-token-pos")
             .help("Drop tokens with a POS tags appended"))
        .arg(Arg::with_name("input")
             .help("File(s) to read from")
             .required(true)
             .index(1)
             .multiple(true))
        .get_matches();

    let tf_threshold =
        match arg_matches.value_of("tf_threshold").unwrap_or("100").parse::<u32>() {
            Ok(x) => x,
            Err(e) => panic!("Bad term frequency threshold: {}", e),
        };
    let df_threshold =
        match arg_matches.value_of("df_threshold").unwrap_or("50").parse::<u32>() {
            Ok(x) => x,
            Err(e) => panic!("Bad document frequency threshold: {}", e),
        };
    let drop_token_pos = arg_matches.occurrences_of("drop_token_pos") > 0;

    // TODO(dstu): make sure we can write to output file before doing all the
    // heavy lifting.
    let output_file = arg_matches.value_of("out")
        .map(|s| {
             let p = path::Path::new(s);
             if p.is_dir() {
                 panic!("Output path cannot be a directory")
             } else {
                 p
             }
        }).unwrap();

    let input_paths: Vec<String> =
        arg_matches.values_of("input").unwrap().map(|x| x.to_string()).collect();

    // let mut message = capnp::message::Builder::new_default();
    // let mut dictionary = message.init_root::<ngram_indexer::Dictionary::Builder>();
    // let mut token_entries = dictionary.init_entries(256);
    // {
    //     let unknown_token = arg_matches.value_of("unknown_token").unwrap_or("*UNK*");
    //     let mut unk_token_entry = ngram_indexer::TokenId::
    //     unk_token_entry.raw_text = unknown_token.clone();
    //     unk_token_entry.id = 0;
    //     dictionary.tokens.push(unk_token_entry);
    // }

    let mut next_id = 1;
    for input_path in input_paths.into_iter() {
        let mut reader = ngram_indexer::tsv_gz_reader(input_path).unwrap();
        let grouped = reader.decode::<ngram_indexer::GoogleBooksNgramEntry>()
            .enumerate()
            .map(|(i, result)|
                 match result {
                     Ok(entry) => entry,
                     Err(e) => panic!("error processing entry on line {}: {:?}", i, e),
                 })
            .group_by_lazy(|entry| entry.ngram.clone());

        for (ngram, entries) in grouped.into_iter() {
            if drop_token_pos && ngram_indexer::token_has_terminal_pos_tag(&ngram) {
                continue
            }
            let (sum_tf, sum_df) = entries.fold(
                (0, 0),
                |(sum_tf, sum_df), entry|
                (sum_tf + entry.term_frequency, sum_df + entry.document_frequency)
            );
            if sum_tf >= tf_threshold && sum_df >= df_threshold {
                // let mut token_entry = ngram_indexer::schema::TokenId::new();
                // token_entry.raw_text = ngram.clone();
                // token_entry.id = next_id;
                // dictionary.tokens.push(token_entry);
                println!("{}: {}", ngram, next_id);
                next_id += 1;
            }
        }
    }

    // TODO(dstu): write dictionary to output file.
}
