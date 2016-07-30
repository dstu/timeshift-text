@0xf28e5917dd7919f5;

struct Ngram {
  tokens @0 :List(UInt32);
}

struct NgramCounts {
  year @0 :UInt16;
  counts @1 :List(Count);

  struct Count {
    ngram @0 :Ngram;
    termFrequency @1 :UInt32;
    documentFrequency @2 :UInt32;
  }
}
