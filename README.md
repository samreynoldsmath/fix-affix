# fix-affix
Build a Hunspell dictionary via TOML

## About
[Hunspell](https://hunspell.github.io/) is a ubiquitous spell checker tool that defines its dictionaries in a dual-file `.dic`/`.aff` format. While Hunspell itself is a flexible and performant tool, the dictionary format leaves something to be desired when it comes to human-readability and maintenance.

`fix-affix` is a command line utility that allows the user to build a Hunspell dictionary from a less-cryptic TOML file. It is a free and open-source tool built in Rust, which I developed to assist me with building a repository of [math words](https://github.com/samreynoldsmath/math-words).

All of the code and documentation was written by a real human, not an LLM.

> [!WARN]
> `fix-affix` is a work-in-progress and cannot yet generate every possible Hunspell library. In particular, word compounding has not yet been implemented.

## Installation

### As a command line utility
```bash
cargo install fix-affix
```

### As a library
```bash
cargo add fix-affix
```

## Usage
Define your dictionary in `my_dict.toml` following the [specifications](#toml-file-specification-for-hunspell-dictionaries), then run
```bash
fix-affix my_dict.toml
```
which will create `my_dict.dic` and `my_dict.aff` in the same directory as `my_dict.toml`:
```text
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
complex_prefixes = true

[[config.input_conversion]]
remove = "’"
add = "'"

[[config.replace]]
remove = "ie"
add = "ai"

[[config.replace]]
remove = "alot"
add = "a lot"

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
```text
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
```text
3
Sam/10,102
act/100,101,103
dismiss/0,103
```

## TOML File Specification for Hunspell Dictionaries

Before learning how to specify a Hunspell library with TOML, you should review the [Hunspell man pages](https://man.archlinux.org/man/hunspell.5.en) to see how `.aff`/`.dic` files are used by Hunspell.

Unless otherwise noted:
- All key-value pairs are optional.
- Any optional key-value pair that is left unspecified will not be added to the `.aff`/`.dic` files (for better or worse)
- Boolean values default to `false`

Where appropriate, the TOML and Hunspell notation is separated by | for easy comparison.

### `metadata`
- `title` (**Required:** String): A short name for your dictionary
- `description` (**Required:** String): A description of your dictionary
- `version` (**Required:** String): The dictionary version, such as SemVer
- `authors` (**Required:** Array of Strings): The authors of the dictionary (and optionally their contact info)

### `config`
- `encoding` (String) | "SET": The character encoding of the dictionary
- `additional_word_characters` (String) | "WORDCHARS": Extends the character set for valid words
- `complex_prefixes` (Boolean) | "COMPLEXPREFIXES": Allow stacking of two prefixes and disable stacking of suffixes
- `language_code` (String) | "LANG": Set language code for language-specific functions of Hunspell
- `ignore_characters` (String) | "IGNORE": Characters that will be ignored in dictionary words
- `try_characters` (String) | "TRY": The order in which characters are substituted to offer spelling suggestions
- `max_n_gram_suggestions` (Unsigned Integer) | "MAXNGRAMSUGS": Maximum number of n-gram suggestions
- `max_diff` (1 -- 10) | "MAXDIFF": Similarity factor for n-gram suggestions
- `only_max_diff` (Boolean) | "ONLYMAXDIFF": Remove all bad n-gram suggestions
- `no_split_suggestions` (Boolean) | "NOSPLITSUGS": Disable word suggestions with spaces
- `suggest_with_dots` (Boolean) | "SUGSWITHDOTS": Add dots to suggestions if input word ends in dots
- `forbid_warn` (Boolean) | "FORBIDWARN": Words with the `warn` | "WARN" flag are not accepted as correctly spelled
- `input_conversion` (Array of Tables) | "ICONV": Defines character conversions prior to applying the spell checker. Each element of the array is a table with two entries, `remove` and `add`, whose values are strings.
- `replace` (Array of Tables): Defines alternative spelling patterns for common misspellings; similar to `metadata.input_conversion`, `replace` is an array of tables, each having an `add` and `remove` field whose values are strings

### `prefix` and `affix`
Affixes come in two flavors: `prefix` | "PFX" and `suffix` | "SFX". We focus on `prefix`, as `suffix` works the same way.

`prefix` is a table of tables. Each sub-table is given a unique name, and defines a how a prefix can be joined to a stem in four key-value pairs:
- `rules` (Array of Tables): Each table in the array consists of:
    - `add` (**Required:** String): The characters to be appended
    - `strip` (String): The characters to be removed prior to appending
    - `cond` (String): The conditions for when the rule can be applied, specified using the same regex notation as Hunspell
    - `stack` (Array of Strings): If `config.complex_prefixes = true`, each string in `stack` is the key of another `prefix` table that is permitted to be secondarily appended; if `config.complex_prefixes = false`, this instead applies to `suffix`
- `cross_product` (Boolean) | "Y"/"N": **True by default.*** If true, this prefix is permitted to be combined with suffixes
- `circumfix` (Boolean): Can be used as part of a circumfix in languages that allow them
- `substandard` (Boolean): Any word with the prefix will not be suggested or used in morphological analysis

### `entry`
`entry` is an array of tables defining words and word stems, along with several flags that determine how they interact with affixes:
- `stem` (String): The word or word stem
- `prefix` (Array of Strings): The keys of each prefix table that can be applied to the stem
- `suffix` (Array of Strings): The keys of each suffix table that can be applied to the stem
- `no_suggest` (Boolean) | "NOSUGGEST" flag: This word will not appear in spelling suggestions
- `warn` (Boolean) | "WARN": For rare words that are often spelling mistakes
- `forbidden_word` (Boolean) | "FORBIDDENWORD" flag: This word will always be marked as misspelled
- `keep_case` (Boolean) | "KEEPCASE" flag: This word will be marked as misspelled unless lower/uppercase letters match exactly
- `need_affix` (Boolean) | "NEEDAFFIX" flag: This stem will be marked as misspelled unless it has a prefix/suffix
- `substandard` (Boolean) | "SUBSTANDARD": This word will not be suggested or used in morphological generation
