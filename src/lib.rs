mod context;
mod process;
mod read;
mod write;

pub use context::ContextManager;
use process::{build_flag_code_look_up, collect_flag_codes, get_sorted_affixes};
pub use read::TomlDict;
pub use read::load_toml_dict;
use read::{Affix, DictEntry, FlagCode, FlagCodeLookup};
pub use write::build_hunspell_dictionary;

const VERSION: &str = "0.1.0";
