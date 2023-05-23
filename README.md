# Numbers into words

Converts an integer into English words.

License: MIT OR Apache-2.0

- [`crates.io`](https://crates.io/crates/numbers_into_words)
- [Documentation (`docs.rs`)](https://docs.rs/crate/numbers_into_words/latest)

## Command line reference

- Usage: `target/debug/numbers_into_words [OPTIONS] [NUMBERS]`

### Options

- `--help`: Display help message

- `--and=`(`none` | `last` | `below1k` | `all` ): Specify when the word "and"
  should be used in phrases like "five-hundred and seventy-two"

- `--and-help`: Describe the options for `--and=`

- `--minimal`: Output only the words for each number (rather than prefacing
  with the numerals, e.g. "five" instead of "5: five")

### "and" options

- `--and=none`: Don't use the word "and" (e.g. "five-hundred seventy-two")

- `--and=last`: Only use the word "and" in the hundreds-tens-units group (e.g.
  "three-hundred five thousand, five-hundred and seventy-two", but
  "three-hundred five million, five-hundred seventy-two thousand")

- `--and=below1k`: Only use the word "and" for numbers below 1000 (e.g.
  "three-hundred five thousand, five-hundred seventy-two", but "three-hundred
  and five")

- `--and=all`: Always use "and" (default behavior) (e.g. "five-hundred and
  twenty-four million, three-hundred and seventy-eight")

### Usage examples

```
$ numbers_into_words 234 92,582,349 543_953_459_343 8
234: two-hundred and thirty-four
92582349: ninety-two million, five-hundred and eighty-two thousand, three-hundred and forty-nine
543953459343: five-hundred and forty-three billion, nine-hundred and fifty-three million, four-hundred and fifty-nine thousand, three-hundred and forty-three
8: eight
```

```
$ numbers_into_words --minimal 593_123 45,230
five-hundred and ninety-three thousand, one-hundred and twenty-three
forty-five thousand, two-hundred and thirty
```

```
$ numbers_into_words --minimal --and=last 532_428_000 1000355
five-hundred thirty-two million, four-hundred twenty-eight thousand
one million, three-hundred and fifty-five
```

```
$ numbers_into_words -and=below1k 400_000_000_123 678
400000000123: four-hundred billion, one-hundred twenty-three
678: six-hundred and seventy-eight
```
