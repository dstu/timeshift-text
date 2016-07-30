@0x829e56c0f0c484a5;

struct TokenId {
  rawText @0 :Text;
  id @1 :UInt32;  # 4.29 billion ought to be enough
}

struct Dictionary {
  entries @0 :List(TokenId);
}
