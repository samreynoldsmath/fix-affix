#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod process;
mod read;
mod write;

pub use read::HunspellDict;
use read::{Affix, CondReplace, DerivedDictData, DictConfig, DictEntry, FlagCode};

const VERSION: &str = "0.1.0";

const REPO_URL: &str = "https://github.com/samreynoldsmath/fix-affix";
const DATE_FMT: &str = "%Y-%m-%d %H:%M";
