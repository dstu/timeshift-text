extern crate csv;
extern crate flate2;
extern crate rustc_serialize;

use std::io;
use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;

#[derive(RustcDecodable)]
pub struct GoogleBooksNgramEntry {
    ngram: String,
    year: u16,
    term_frequency: u32,
    document_frequency: u32,
}

pub fn tsv_gz_reader<P: AsRef<Path>>(path: P) -> io::Result<csv::Reader<GzDecoder<File>>> {
    Ok(csv::Reader::from_reader(try!(GzDecoder::new(try!(File::open(path))))))
}

pub fn tsv_reader<P: AsRef<Path>>(path: P) -> io::Result<csv::Reader<File>> {
    Ok(csv::Reader::from_reader(try!(GzDecoder::new(try!(File::open(path))))))
}
