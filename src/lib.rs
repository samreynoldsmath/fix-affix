mod process;
mod read;
mod write;

use process::{build_flag_code_look_up, collect_flag_codes};
pub use read::TomlDict;
pub use read::load_toml_dict;
use read::{Affix, DictEntry, FlagCode, FlagCodeLookup};
pub use write::build_hunspell_dictionary;
