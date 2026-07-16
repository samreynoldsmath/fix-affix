mod context;
mod process;
mod read;
mod write;

pub use context::ContextManager;
pub use read::HunspellDict;
use read::{Affix, DerivedDictData, DictEntry, FlagCode};

const VERSION: &str = "0.1.0";
