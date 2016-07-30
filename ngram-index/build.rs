extern crate capnpc;

fn main() {
    ::capnpc::compile("schema", &["src/schema/dictionary.capnp",
                                  "src/schema/ngram_counts.capnp",]).unwrap();
}
