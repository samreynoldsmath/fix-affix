use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

/// Contains the Hunspell library data
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct HunspellDict {
    pub(crate) metadata: DictMetadata,
    pub(crate) config: DictConfig,
    pub(crate) prefix: HashMap<String, Affix>,
    pub(crate) suffix: HashMap<String, Affix>,
    pub(crate) entry: Vec<DictEntry>,
    #[serde(skip)]
    pub(crate) derived: DerivedDictData,
}

#[derive(Debug, Default)]
pub(crate) struct DerivedDictData {
    pub(crate) sorted_prefix_keys: Vec<String>,
    pub(crate) sorted_suffix_keys: Vec<String>,
    pub(crate) code_map: CodeMap,
    pub(crate) used_flags: Vec<FlagCode>,
}

#[derive(Debug, Default)]
pub(crate) struct CodeMap {
    pub(crate) cfg_map: Vec<(String, FlagCode)>,
    pub(crate) pfx_map: HashMap<String, FlagCode>,
    pub(crate) sfx_map: HashMap<String, FlagCode>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct DictConfig {
    pub(crate) encoding: String,
    pub(crate) complex_prefixes: bool,
    pub(crate) language_code: String,
    pub(crate) max_n_gram_suggestions: u8,
    pub(crate) max_diff: u8,
    pub(crate) only_max_diff: bool,
    pub(crate) no_split_suggestions: bool,
    pub(crate) suggest_with_dots: bool,
    pub(crate) forbid_warn: bool,
    pub(crate) full_strip: bool,
    pub(crate) check_sharps: bool,
    pub(crate) characters: CharacterConfig,
    pub(crate) compound: CompoundConfig,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct CharacterConfig {
    pub(crate) additional: String,
    pub(crate) ignore: String,
    pub(crate) try_order: String,
    pub(crate) key_groups: Vec<String>,
    pub(crate) remap: Vec<Replace>,
    pub(crate) try_replace: Vec<Replace>,
    pub(crate) phonetic_replace: Vec<Replace>,
    pub(crate) input_conversion: Vec<Replace>,
    pub(crate) output_conversion: Vec<Replace>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct CompoundConfig {
    pub(crate) check_case: bool,
    pub(crate) check_duplicate: bool,
    pub(crate) check_replace: bool,
    pub(crate) check_triple: bool,
    pub(crate) more_suffixes: bool,
    pub(crate) simplified_triple: bool,
    pub(crate) min_char: u8,
    pub(crate) max_word: u8,
    pub(crate) max_suggestions: u8,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DictMetadata {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) version: String,
    pub(crate) authors: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct Replace {
    pub(crate) remove: String,
    pub(crate) add: String,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct DictEntry {
    pub(crate) stem: String,
    pub(crate) prefix: Vec<String>,
    pub(crate) suffix: Vec<String>,
    pub(crate) no_suggest: bool,
    pub(crate) warn: bool,
    pub(crate) forbidden_word: bool,
    pub(crate) keep_case: bool,
    pub(crate) need_affix: bool,
    pub(crate) substandard: bool,
    pub(crate) compound: bool,
    pub(crate) compound_begin: bool,
    pub(crate) compound_middle: bool,
    pub(crate) compound_last: bool,
    pub(crate) compound_force_uppercase: bool,
    pub(crate) compound_root: bool,
    pub(crate) compound_only: bool,
    pub(crate) compound_forbid: bool,
}

#[derive(Debug)]
pub enum AffixType {
    Prefix,
    Suffix,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct AffixRule {
    pub(crate) add: String,
    pub(crate) strip: String,
    pub(crate) cond: String,
    pub(crate) stack: Vec<String>,
    pub(crate) circumfix: bool,
    pub(crate) substandard: bool,
    pub(crate) compound: bool,
    pub(crate) compound_begin: bool,
    pub(crate) compound_middle: bool,
    pub(crate) compound_last: bool,
    pub(crate) compound_only: bool,
    pub(crate) compound_interior: bool,
    pub(crate) compound_forbid: bool,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct Affix {
    pub(crate) rules: Vec<AffixRule>,
    #[serde(default = "bool_true")]
    pub(crate) cross_product: bool,
}

fn bool_true() -> bool {
    true
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct FlagCode(pub u16);
impl Display for FlagCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
