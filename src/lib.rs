#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod process;
mod read;
mod types;
mod write;

pub use types::HunspellDict;
use types::{Affix, CodeMap, CondReplace, DerivedDictData, DictConfig, DictEntry, FlagCode};

const VERSION: &str = "0.1.1";

const REPO_URL: &str = "https://github.com/samreynoldsmath/fix-affix";
const DATE_FMT: &str = "%Y-%m-%d %H:%M";
