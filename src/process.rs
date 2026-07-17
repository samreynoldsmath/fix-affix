use crate::{Affix, DerivedDictData, DictEntry, FlagCode, HunspellDict};
use anyhow::{Error, Result};
use std::collections::HashMap;

impl HunspellDict {
    pub(crate) fn compute_derived_data(&mut self) -> Result<()> {
        let sorted_prefix: Vec<String> = get_sorted_affix_keys(&self.prefix);
        let sorted_suffix: Vec<String> = get_sorted_affix_keys(&self.suffix);
        let flag_codes: HashMap<String, FlagCode> =
            build_flag_code_look_up(&sorted_prefix, &sorted_suffix)?;
        let used_flags: Vec<FlagCode> = self.get_used_flags(&flag_codes);
        self.derived = DerivedDictData {
            sorted_prefix,
            sorted_suffix,
            flag_codes,
            used_flags,
        };
        Ok(())
    }

    pub(crate) fn get_used_flags(&self, flag_codes: &HashMap<String, FlagCode>) -> Vec<FlagCode> {
        let mut used_flags: Vec<FlagCode> = vec![];
        for word in &self.entry {
            let codes: Vec<FlagCode> = word.collect_flag_codes(flag_codes);
            for code in codes {
                if code.0 >= 100 {
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

fn get_sorted_affix_keys(affixes: &HashMap<String, Affix>) -> Vec<String> {
    let mut vec_affix: Vec<String> = affixes.keys().cloned().collect();
    vec_affix.sort();
    vec_affix
}

fn build_flag_code_look_up(
    prefixes: &Vec<String>,
    suffixes: &Vec<String>,
) -> Result<HashMap<String, FlagCode>> {
    let k: usize = prefixes.len();

    let total_num_flags: usize = 100 + k + suffixes.len();
    if total_num_flags > 65_000 {
        let e: Error = Error::msg("Total number of flags cannot exceed 65,000");
        return Err(e);
    }

    let mut flag_codes: HashMap<String, FlagCode> = HashMap::new();

    flag_codes.insert("{no_suggest}".to_string(), FlagCode(1));
    flag_codes.insert("{warn}".to_string(), FlagCode(2));
    flag_codes.insert("{forbidden_word}".to_string(), FlagCode(3));
    flag_codes.insert("{keep_case}".to_string(), FlagCode(10));
    flag_codes.insert("{need_affix}".to_string(), FlagCode(11));
    flag_codes.insert("{substandard}".to_string(), FlagCode(12));

    let prefix_start: u16 = 100;
    for (i, p) in (prefix_start..).zip(prefixes) {
        flag_codes.insert(p.to_string(), FlagCode(i));
    }

    let suffix_start: u16 = (100 + k) as u16;
    for (i, p) in (suffix_start..).zip(suffixes) {
        flag_codes.insert(p.to_string(), FlagCode(i));
    }

    Ok(flag_codes)
}

impl DictEntry {
    pub(crate) fn collect_flag_codes(
        &self,
        flag_codes: &HashMap<String, FlagCode>,
    ) -> Vec<FlagCode> {
        let mut entry_codes: Vec<FlagCode> = vec![];
        if self.no_suggest {
            entry_codes.push(FlagCode(1));
        }
        if self.warn {
            entry_codes.push(FlagCode(2));
        }
        if self.forbidden_word {
            entry_codes.push(FlagCode(3));
        }
        if self.keep_case {
            entry_codes.push(FlagCode(10));
        }
        if self.need_affix {
            entry_codes.push(FlagCode(11));
        }
        if self.substandard {
            entry_codes.push(FlagCode(12));
        }

        for p in &self.prefix {
            if !flag_codes.contains_key(p) {
                panic!("No flag code for {}", p);
            }
            let code: FlagCode = flag_codes[p];
            entry_codes.push(code);
        }

        for s in &self.suffix {
            if !flag_codes.contains_key(s) {
                panic!("No flag code for {}", s);
            }
            let code: FlagCode = flag_codes[s];
            entry_codes.push(code);
        }

        entry_codes
    }
}
