use anyhow::Result;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};
use toml::value::Date;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictMetadata {
    title: String,
    description: String,
    version: String,
    date: Date,
    authors: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TomlDict {
    metadata: DictMetadata,
    config: Option<DictConfig>,
    prefix: Option<HashMap<String, Affix>>,
    suffix: Option<HashMap<String, Affix>>,
    replace: Option<Vec<Replace>>,
    entry: Option<Vec<DictEntry>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictConfig {
    encoding: Option<String>,
    flag_type: Option<String>,
    #[serde(default)]
    complex_prefixes: bool,
    language_code: Option<String>,
    ignore_characters: Option<Vec<String>>,
    try_characters: Option<String>,
    max_compound_suggestions: Option<u8>,
    max_n_gram_suggestions: Option<u8>,
    max_n_gram_diff: Option<u8>,
    max_diff: Option<u8>,
    only_max_diff: Option<u8>,
    #[serde(default)]
    no_split_suggestions: bool,
    #[serde(default)]
    suggest_with_dots: bool,
    input_conversion: Option<Vec<Replace>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Replace {
    remove: String,
    add: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DictEntry {
    stem: String,
    prefix: Option<Vec<String>>,
    suffix: Option<Vec<String>>,
    #[serde(default)]
    no_suggest: bool,
    #[serde(default)]
    warn: bool,
    #[serde(default)]
    forbid_warn: bool,
    #[serde(default)]
    compound_flag: bool,
    #[serde(default)]
    compound_begin: bool,
    #[serde(default)]
    compound_last: bool,
    #[serde(default)]
    compound_middle: bool,
    #[serde(default)]
    only_in_compound: bool,
    #[serde(default)]
    compound_permit_flag: bool,
    #[serde(default)]
    forbidden_word: bool,
    #[serde(default)]
    keep_case: bool,
    #[serde(default)]
    need_affix: bool,
    #[serde(default)]
    substandard: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CondReplace {
    strip: Option<String>,
    add: Option<String>,
    cond: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Affix {
    rules: Vec<CondReplace>,
    #[serde(default)]
    cross_product: bool,
    #[serde(default)]
    circum_fix: bool,
    #[serde(default)]
    substandard: bool,
}

pub fn load_toml_dict(path: &Path) -> Result<TomlDict> {
    let raw: String = fs::read_to_string(path)?;
    let dict: TomlDict = toml::from_str(&raw)?;
    Ok(dict)
}
