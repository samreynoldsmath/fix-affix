# fix-affix
Build a Hunspell dictionary via TOML

## About
[Hunspell](https://hunspell.github.io/) is a ubiquitous spell checker tool that defines its dictionaries in a dual-file `.dic`/`.aff` format. While Hunspell itself is a flexible and performant tool, the dictionary format leaves something to be desired when it comes to human-readability and maintenance.

`fix-affix` is a command line tool that allows the user to build a Hunspell dictionary from a less-cryptic TOML file. It is a free and open-source tool built in Rust, which I developed to assist me with building a repository of [math words](https://github.com/samreynoldsmath/math-words).

All of the code and documentation was written by a real human, not an LLM.

## Installation

### As a Standalone Tool (Rust Binary)
```bash
cargo install fix-affix
```

### As Part of a Rust Crate
```bash
cargo add fix-affix
```

## Usage
Define your dictionary in `my_dict.toml` following the [specifications](TODO), then run
```bash
fix-affix my_dict.toml
```
which will create `my_dict.dic` and `my_dict.aff` in the same directory as `my_dict.toml`:
```
.
├── my_dict.aff
├── my_dict.dic
└── my_dict.toml
```

## Example
Define your dictionary in `my_dict.toml`:
```toml
[metadata]
title = "ex_dict"
description = "Example Dictionary"
version = "1.2.3"
authors = [
    "Mickey Mouse (mickey@example.com)",
    "Donald Duck (github.com/donald_duck)",
]

[config]
encoding = "UTF-8"
additional_word_characters = "'"
try_characters = "esianrtolcdugmphbyfvkwzESIANRTOLCDUGMPHBYFVKWZ'"
input_conversion = [{remove = "’", add = "'"}]
complex_prefixes = true

[prefix.non]
rules = [{add = "non"}]

[prefix.re]
rules = [{add = "re", stack = ["non"]}]

[suffix."'s"]
cross_product = false
rules = [{add = "'s"}]

[suffix.ive]
rules = [
    {strip = "e", add = "ive", cond = "e"},
    {add = "ive", cond = "[^e]"},
]

[[replace]]
remove = "ie"
add = "ai"

[[replace]]
remove = "alot"
add = "a lot"

[[entry]]
stem = "Sam"
keep_case = true
suffix = ["'s"]

[[entry]]
stem = "act"
prefix = ["non", "re"]
suffix = ["ive"]

[[entry]]
stem = "dismiss"
no_suggest = true
suffix = ["ive"]
```

Run
```bash
fix-affix my_dict.toml
```

The contents of `my_dict.aff`:
```
FLAG num
SET UTF-8
WORDCHARS '
COMPLEXPREFIXES
TRY esianrtolcdugmphbyfvkwzESIANRTOLCDUGMPHBYFVKWZ'
ICONV 1
ICONV ’ '
NOSUGGEST 0
KEEPCASE 10

PFX 100 Y 1
PFX 100   0 non .

PFX 101 Y 1
PFX 101   0 re/100 .

SFX 102 N 1
SFX 102   0 's .

SFX 103 Y 2
SFX 103   e ive e
SFX 103   0 ive [^e]

REP 2
REP ie ai
REP alot a_lot
```

The contents of `my_dict.dic`:
```
3
Sam/10,102
act/100,101,103
dismiss/0,103
```
