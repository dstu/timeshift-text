extern crate capnpc;

fn main() {
    ::capnpc::compile("schema", &["src/schema/ngram_index.capnp"]).unwrap();
}
