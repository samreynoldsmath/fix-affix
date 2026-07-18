use crate::{Affix, CodeMap, DerivedDictData, DictEntry, FlagCode, HunspellDict};
use anyhow::{Error, Result};
use std::collections::HashMap;

const MAX_CONFIG_CODE: u16 = 100;
const MAX_FLAGS: usize = 65_000;

fn get_config_code_map() -> Vec<(String, FlagCode)> {
    let mut flag_codes: Vec<(String, FlagCode)> = vec![];

    for (key, i) in [
        ("NOSUGGEST", 1),
        ("SUBSTANDARD", 2),
        ("WARN", 3),
        ("FORBIDDENWORD", 4),
        ("KEEPCASE", 10),
        ("NEEDAFFIX", 11),
    ] {
        flag_codes.push((key.to_string(), FlagCode(i)));
    }

    flag_codes
}

impl HunspellDict {
    pub(crate) fn compute_derived_data(&mut self) -> Result<()> {
        let sorted_prefix_keys: Vec<String> = get_sorted_affix_keys(&self.prefix);
        let sorted_suffix_keys: Vec<String> = get_sorted_affix_keys(&self.suffix);

        let code_map: CodeMap = CodeMap::new(&sorted_prefix_keys, &sorted_suffix_keys)?;

        let used_flags: Vec<FlagCode> = self.get_used_flags(&code_map);

        self.derived = DerivedDictData {
            sorted_prefix_keys,
            sorted_suffix_keys,
            code_map,
            used_flags,
        };
        Ok(())
    }

    fn get_used_flags(&self, code_map: &CodeMap) -> Vec<FlagCode> {
        let mut used_flags: Vec<FlagCode> = vec![];
        for word in &self.entry {
            let codes: Vec<FlagCode> = word.collect_flag_codes(code_map);
            for code in codes {
                if code.0 >= MAX_CONFIG_CODE {
                    continue;
                }
                if !used_flags.contains(&code) {
                    used_flags.push(code);
                }
            }
        }
        used_flags.sort_by_key(|code| code.0);
        used_flags
    }
}

impl CodeMap {
    fn new(sorted_prefix_keys: &Vec<String>, sorted_suffix_keys: &Vec<String>) -> Result<CodeMap> {
        let num_pfx: usize = sorted_prefix_keys.len();
        let num_sfx: usize = sorted_suffix_keys.len();

        let total_num_flags: usize = MAX_CONFIG_CODE as usize + num_pfx + num_sfx;
        if total_num_flags > MAX_FLAGS {
            let e: Error = Error::msg("Total number of flags cannot exceed 65,000");
            return Err(e);
        }

        let cfg_map: Vec<(String, FlagCode)> = get_config_code_map();

        let pfx_map: HashMap<String, FlagCode> =
            get_affix_code_map(sorted_prefix_keys, MAX_CONFIG_CODE);

        let sfx_map: HashMap<String, FlagCode> =
            get_affix_code_map(sorted_suffix_keys, MAX_CONFIG_CODE + num_pfx as u16);

        Ok(CodeMap {
            cfg_map,
            pfx_map,
            sfx_map,
        })
    }
}

fn get_sorted_affix_keys(affixes: &HashMap<String, Affix>) -> Vec<String> {
    let mut vec_affix: Vec<String> = affixes.keys().cloned().collect();
    vec_affix.sort();
    vec_affix
}

fn get_affix_code_map(
    sorted_affix_keys: &Vec<String>,
    index_start: u16,
) -> HashMap<String, FlagCode> {
    let mut flag_codes: HashMap<String, FlagCode> = HashMap::new();
    for (i, a) in (index_start..).zip(sorted_affix_keys) {
        flag_codes.insert(a.to_string(), FlagCode(i));
    }
    flag_codes
}

impl DictEntry {
    pub(crate) fn collect_flag_codes(&self, code_map: &CodeMap) -> Vec<FlagCode> {
        let mut entry_codes: Vec<FlagCode> = vec![];

        for (key, code) in &code_map.cfg_map {
            if self.match_entry_option_to_key(key) {
                entry_codes.push(*code);
            }
        }

        for p in &self.prefix {
            if !code_map.pfx_map.contains_key(p) {
                panic!("Unknown prefix key: {}", p);
            }
            entry_codes.push(code_map.pfx_map[p]);
        }

        for s in &self.suffix {
            if !code_map.sfx_map.contains_key(s) {
                panic!("Unknown suffix key: {}", s);
            }
            entry_codes.push(code_map.sfx_map[s]);
        }

        entry_codes
    }

    fn match_entry_option_to_key(&self, key: &str) -> bool {
        (self.no_suggest && key == "NOSUGGEST")
            || (self.warn && key == "WARN")
            || (self.forbidden_word && key == "FORBIDDENWORD")
            || (self.keep_case && key == "KEEPCASE")
            || (self.need_affix && key == "NEEDAFFIX")
            || (self.substandard && key == "SUBSTANDARD")
    }
}
