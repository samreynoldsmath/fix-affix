use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;
use toml::value::Date;

pub fn load_toml_dict(path: &Path) -> Result<TomlDict> {
    let raw: String = std::fs::read_to_string(path)?;
    let dict: TomlDict = toml::from_str(&raw)?;
    Ok(dict)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TomlDict {
    pub(crate) metadata: DictMetadata,
    pub(crate) config: Option<DictConfig>,
    pub(crate) prefix: Option<HashMap<String, Affix>>,
    pub(crate) suffix: Option<HashMap<String, Affix>>,
    #[allow(dead_code)]
    pub(crate) replace: Option<Vec<Replace>>, // TODO
    pub(crate) entry: Option<Vec<DictEntry>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DictConfig {
    pub(crate) encoding: Option<String>,
    pub(crate) additional_word_characters: Option<String>,
    #[serde(default)]
    pub(crate) complex_prefixes: bool,
    pub(crate) language_code: Option<String>,
    pub(crate) ignore_characters: Option<String>,
    pub(crate) try_characters: Option<String>,
    pub(crate) max_compound_suggestions: Option<u8>,
    pub(crate) max_n_gram_suggestions: Option<u8>,
    pub(crate) max_diff: Option<u8>,
    #[serde(default)]
    pub(crate) only_max_diff: bool,
    #[serde(default)]
    pub(crate) no_split_suggestions: bool,
    #[serde(default)]
    pub(crate) suggest_with_dots: bool,
    pub(crate) input_conversion: Option<Vec<Replace>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DictMetadata {
    // TODO make fields optional
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) version: String,
    pub(crate) date: Date,
    pub(crate) authors: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Replace {
    pub(crate) remove: String,
    pub(crate) add: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct DictEntry {
    pub(crate) stem: String,
    pub(crate) prefix: Option<Vec<String>>,
    pub(crate) suffix: Option<Vec<String>>,
    #[serde(default)]
    pub(crate) no_suggest: bool,
    #[serde(default)]
    pub(crate) warn: bool,
    #[serde(default)]
    pub(crate) forbid_warn: bool,
    #[serde(default)]
    pub(crate) compound_flag: bool,
    #[serde(default)]
    pub(crate) compound_begin: bool,
    #[serde(default)]
    pub(crate) compound_last: bool,
    #[serde(default)]
    pub(crate) compound_middle: bool,
    #[serde(default)]
    pub(crate) only_in_compound: bool,
    #[serde(default)]
    pub(crate) compound_permit_flag: bool,
    #[serde(default)]
    pub(crate) forbidden_word: bool,
    #[serde(default)]
    pub(crate) keep_case: bool,
    #[serde(default)]
    pub(crate) need_affix: bool,
    #[serde(default)]
    pub(crate) substandard: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct CondReplace {
    pub(crate) strip: Option<String>,
    pub(crate) add: Option<String>,
    pub(crate) cond: Option<String>,
    pub(crate) stack: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Affix {
    pub(crate) rules: Vec<CondReplace>,
    #[serde(default)]
    pub(crate) cross_product: bool,
    #[serde(default)]
    #[allow(dead_code)]
    pub(crate) circum_fix: bool, // TODO
    #[serde(default)]
    #[allow(dead_code)]
    pub(crate) substandard: bool, // TODO
}

#[derive(Clone, Copy)]
pub(crate) struct FlagCode(pub u16);
impl Display for FlagCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) type FlagCodeLookup = HashMap<String, FlagCode>;
