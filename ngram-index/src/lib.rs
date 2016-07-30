extern crate capnp;
extern crate csv;
extern crate flate2;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rustc_serialize;

use std::io;
use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;
use regex::Regex;

// include!(concat!(env!("OUT_DIR"), "/dictionary_capnp.rs"));
// include!(concat!(env!("OUT_DIR"), "/ngram_counts_capnp.rs"));

/// Fields of a Google Books historical n-gram statistics TSV file.
#[derive(RustcDecodable)]
pub struct GoogleBooksNgramEntry {
    pub ngram: String,
    pub year: u16,
    pub term_frequency: u32,
    pub document_frequency: u32,
}

/// Opens the gzipped file at `p` and wraps a CSV reader around it.
pub fn tsv_gz_reader<P: AsRef<Path>>(path: P) -> io::Result<csv::Reader<GzDecoder<File>>> {
    Ok(csv::Reader::from_reader(try!(GzDecoder::new(try!(File::open(path)))))
       .has_headers(false)
       .delimiter(b'\t'))
}

/// Opens the file at `p` and wraps a CSV reader around it.
pub fn tsv_reader<P: AsRef<Path>>(path: P) -> io::Result<csv::Reader<File>> {
    Ok(csv::Reader::from_reader((try!(File::open(path))))
       .has_headers(false)
       .delimiter(b'\t'))
}

/// Returns true iff `s` terminates with a part of speech tag.
///
/// This does not treat the start- or end-of-sentence tokens `_START_` or
/// `_END_` as part of speech tags.
pub fn token_has_terminal_pos_tag(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("_(?:NOUN|VERB|X|ADJ|ADV|PRON|DET|ADP|NUM|CONJ|PRT|ROOT)$").unwrap();
    }
    RE.is_match(s)
}

/// Returns true iff `s` is a part of speech tag stand-in.
///
/// This does not treat the start- or end-of-sentence tokens `_START_` or
/// `_END_` as part of speech tags.
pub fn token_is_pos_tag(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^_(?:NOUN|VERB|X|ADJ|ADV|PRON|DET|ADP|NUM|CONJ|PRT|ROOT)_$").unwrap();
    }
    RE.is_match(s)
}
