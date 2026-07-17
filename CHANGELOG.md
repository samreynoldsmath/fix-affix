# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [Unreleased]

### Fixed
- Prevent zero ("0") flags


## [0.1.0] - 2026-07-16

### Added
- Add `HunspellDict` struct to represent a Hunspell dictionary
- Add methods `HunspellDict::load_from_toml_string` and `Hunspell::load_from_toml_file` to build a Hunspell dictionary from a TOML formatted string or file, respectively
- Add methods `HunspellDict::build_dic_string` and `HunspellDict::write_dic_file` for writing the `.dic` file
- Add methods `HunspellDict::build_aff_string` and `HunspellDict::write_aff_file` for writing the `.aff` file
- Add minimal documentation