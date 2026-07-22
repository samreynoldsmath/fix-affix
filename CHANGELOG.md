# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Added
- [ ] Add support for compounding:
    - [x] CHECKCOMPOUNDCASE
    - [x] CHECKCOMPOUNDDUP
    - [ ] CHECKCOMPOUNDPATTERN
    - [x] CHECKCOMPOUNDREP
    - [x] CHECKCOMPOUNDTRIPLE
    - [x] COMPOUNDMIN
    - [x] COMPOUNDMORESUFFIXES
    - [ ] COMPOUNDRULE
    - [ ] COMPOUNDSYLLABLE
    - [x] COMPOUNDWORDMAX
    - [x] MAXCPDSUGS
    - [x] SIMPLIFIEDTRIPLE
    - [ ] SYLLABLENUM
    - [ ] COMPOUNDFLAG
    - [ ] COMPOUNDFORBIDFLAG
    - [ ] COMPOUNDBEGIN
    - [ ] COMPOUNDLAST
    - [ ] COMPOUNDMIDDLE
    - [ ] COMPOUNDPERMITFLAG
    - [ ] COMPOUNDROOT
    - [ ] FORCECASE
    - [ ] ONLYINCOMPOUND

### Changed
- **Breaking:** The following TOML parameters have been renamed:
    - `config.additional_word_characters` -> `config.characters.additional`
    - `config.ignore_characters` -> `config.characters.ignore`
    - `config.try_characters` -> `config.characters.try_order`
    - `config.key_characters` -> `config.characters.key_groups`
    - `config.map_characters` -> `config.characters.remap`
    - `config.phonetic_replace` -> `config.characters.phonetic_replace`
    - `config.input_conversion` -> `config.characters.input_conversion`
    - `config.output_conversion` -> `config.characters.output_conversion`


## [0.2.0]

### Added
- Add support for keywords:
    - CHECKSHARPS
    - FULLSTRIP
    - KEY
    - MAP
    - OCONV
    - PHONE
- Add `clap` to dependencies
- Add support for `--help` and `--version` flags

### Changed
- Clarify in README which keywords are and are not supported
- Update example to reflect changes
- Improve error handling, avoid panicking


## [0.1.1]

### Fixed
- Prevent zero ("0") flags
- Prevent ambiguous flags when prefix and suffix have same label


## [0.1.0] - 2026-07-16

### Added
- Add `HunspellDict` struct to represent a Hunspell dictionary
- Add methods `HunspellDict::load_from_toml_string` and `Hunspell::load_from_toml_file` to build a Hunspell dictionary from a TOML formatted string or file, respectively
- Add methods `HunspellDict::build_dic_string` and `HunspellDict::write_dic_file` for writing the `.dic` file
- Add methods `HunspellDict::build_aff_string` and `HunspellDict::write_aff_file` for writing the `.aff` file
- Add minimal documentation