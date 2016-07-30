@0x829e56c0f0c484a5;  # unique file ID from `capnp id`

# Should we break these up into different files (one for the dictionary, one for
# the ngram counts)?

struct TokenId {
  rawText @0 :Text;
  id @1 :UInt32;  # 4.29 billion ought to be enough
}

struct Dictionary {
  entries @0 :List(TokenId);
}

struct Ngram {
  tokens @0 :List(UInt32);
}

struct NgramCount {
  ngram @0 :Ngram;
  termFrequency @1 :UInt32;
  documentFrequency @2 :UInt32;
}

struct NgramCounts {
  year @0 :UInt16;
  counts @1 :NgramCount;
}
