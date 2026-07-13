# fix-affix
Build a Hunspell dictionary via TOML

## About
[Hunspell](https://hunspell.github.io/) is a ubiquitous spell checker tool that defines its dictionaries in a dual-file `.dic`/`.aff` format. While Hunspell itself is a flexible and performant tool, the dictionary format leaves something to be desired when it comes to human-readability and maintenance.

`fix-affix` is a command line tool that allows the user to build a Hunspell dictionary from a less-cryptic TOML file. It is a free and open-source tool built in Rust, which I developed to assist me with building a repository of [math words](https://github.com/samreynoldsmath/math-words).

All of the code and documentation was written by a real human, not an LLM.

## Installation

### Build from Source
TODO

### Install via Cargo
```bash
cargo install fix-affix
```

## Usage
See [the spec file](doc/spec.toml) to see how to write your dictionary in TOML.
Run
```bash
fix-affix my_dict.toml
```
which will create `my_dict.dic`/`my_dict.aff` in the same directory as `my_dict.toml`:
```
.
├── my_dict.aff
├── my_dict.dic
└── my_dict.toml
