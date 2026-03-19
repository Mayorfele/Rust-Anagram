# anagram-solver

A CLI anagram solver in Rust backed by the English Open Word List.

---

## How It Works

Loads every word from a CSV dictionary at startup and indexes them by their sorted character signature. Two words that share the same signature are anagrams of each other.

```
"listen" → sorted → "eilnst"
"silent" → sorted → "eilnst"
→ same key, same bucket
```

Lookup is O(1) — sort the input word, hit the HashMap, return the bucket.

---

## Running

Point `DICTIONARY_FOLDER` in `main.rs` to your local copy of the [English Open Word List](https://github.com/dwyl/english-words) CSV directory, then:

```bash
cargo run
```

```
Building anagram solver...
Anagram solver ready!

Select an option:
1. Enter word
2. Exit

Enter word to find its anagrams: listen
Anagrams of 'listen': enlist, inlets, listen, silent, tinsel
```

---

## Stack

Rust, csv crate, HashMap
