use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;

pub fn load_toml_dict(path: &Path) -> Result<TomlDict> {
    let raw: String = std::fs::read_to_string(path)?;
    let dict: TomlDict = toml::from_str(&raw)?;
    Ok(dict)
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct TomlDict {
    pub(crate) metadata: DictMetadata,
    pub(crate) config: DictConfig,
    pub(crate) prefix: HashMap<String, Affix>,
    pub(crate) suffix: HashMap<String, Affix>,
    pub(crate) replace: Vec<Replace>,
    pub(crate) entry: Vec<DictEntry>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct DictConfig {
    pub(crate) encoding: String,
    pub(crate) additional_word_characters: String,
    pub(crate) complex_prefixes: bool,
    pub(crate) language_code: String,
    pub(crate) ignore_characters: String,
    pub(crate) try_characters: String,
    pub(crate) max_compound_suggestions: u8,
    pub(crate) max_n_gram_suggestions: u8,
    pub(crate) max_diff: u8,
    pub(crate) only_max_diff: bool,
    pub(crate) no_split_suggestions: bool,
    pub(crate) suggest_with_dots: bool,
    pub(crate) input_conversion: Vec<Replace>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DictMetadata {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) version: String,
    pub(crate) authors: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
    pub(crate) forbid_warn: bool,
    pub(crate) compound_flag: bool,
    pub(crate) compound_begin: bool,
    pub(crate) compound_last: bool,
    pub(crate) compound_middle: bool,
    pub(crate) only_in_compound: bool,
    pub(crate) compound_permit_flag: bool,
    pub(crate) forbidden_word: bool,
    pub(crate) keep_case: bool,
    pub(crate) need_affix: bool,
    pub(crate) substandard: bool,
    pub(crate) circum_fix: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct CondReplace {
    pub(crate) add: String,
    pub(crate) strip: Option<String>,
    pub(crate) cond: Option<String>,
    pub(crate) stack: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[serde(deny_unknown_fields, default)]
pub(crate) struct Affix {
    pub(crate) rules: Vec<CondReplace>,
    #[serde(default = "bool_true")]
    pub(crate) cross_product: bool,
    pub(crate) circum_fix: bool,
    pub(crate) substandard: bool,
}

fn bool_true() -> bool {
    true
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct FlagCode(pub u16);
impl Display for FlagCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) type FlagCodeLookup = HashMap<String, FlagCode>;
