use crate::{Affix, DictEntry, FlagCode, FlagCodeLookup};
use anyhow::{Error, Result};
use std::collections::HashMap;

pub(crate) fn get_used_flags(entries: &[DictEntry], flag_codes: &FlagCodeLookup) -> Vec<FlagCode> {
    let mut used_flags: Vec<FlagCode> = vec![];
    for entry in entries {
        let codes: Vec<FlagCode> = collect_flag_codes(entry, flag_codes);
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

pub(crate) fn get_sorted_affixes(affixes: &HashMap<String, Affix>) -> Vec<(&String, &Affix)> {
    let mut vec_affix: Vec<(&String, &Affix)> = affixes.iter().collect();
    vec_affix.sort_by_key(|x| x.0);
    vec_affix
}

pub(crate) fn build_flag_code_look_up(
    prefixes: &Vec<(&String, &Affix)>,
    suffixes: &Vec<(&String, &Affix)>,
) -> Result<FlagCodeLookup> {
    let k: usize = prefixes.len();

    let total_num_flags: usize = 100 + k + suffixes.len();
    if total_num_flags > 65_000 {
        let e: Error = Error::msg("Total number of flags cannot exceed 65,000");
        return Err(e);
    }

    let mut flag_codes: FlagCodeLookup = HashMap::new();

    flag_codes.insert("{no_suggest}".to_string(), FlagCode(0));
    flag_codes.insert("{warn}".to_string(), FlagCode(1));
    flag_codes.insert("{forbid_warn}".to_string(), FlagCode(2));
    flag_codes.insert("{compound_flag}".to_string(), FlagCode(3));
    flag_codes.insert("{compound_begin}".to_string(), FlagCode(4));
    flag_codes.insert("{compound_last}".to_string(), FlagCode(5));
    flag_codes.insert("{compound_middle}".to_string(), FlagCode(6));
    flag_codes.insert("{only_in_compound}".to_string(), FlagCode(7));
    flag_codes.insert("{compound_permit_flag}".to_string(), FlagCode(8));
    flag_codes.insert("{forbidden_word}".to_string(), FlagCode(9));
    flag_codes.insert("{keep_case}".to_string(), FlagCode(10));
    flag_codes.insert("{need_affix}".to_string(), FlagCode(11));
    flag_codes.insert("{substandard}".to_string(), FlagCode(12));
    flag_codes.insert("{circum_fix}".to_string(), FlagCode(13));

    let prefix_start: u16 = 100;
    for (i, p) in (prefix_start..).zip(prefixes) {
        flag_codes.insert(p.0.to_string(), FlagCode(i));
    }

    let suffix_start: u16 = (100 + k) as u16;
    for (i, p) in (suffix_start..).zip(suffixes) {
        flag_codes.insert(p.0.to_string(), FlagCode(i));
    }

    Ok(flag_codes)
}

pub(crate) fn collect_flag_codes(entry: &DictEntry, flag_codes: &FlagCodeLookup) -> Vec<FlagCode> {
    let mut entry_codes: Vec<FlagCode> = vec![];
    if entry.no_suggest {
        entry_codes.push(FlagCode(0));
    }
    if entry.warn {
        entry_codes.push(FlagCode(1));
    }
    if entry.forbid_warn {
        entry_codes.push(FlagCode(2));
    }
    if entry.compound_flag {
        entry_codes.push(FlagCode(3));
    }
    if entry.compound_begin {
        entry_codes.push(FlagCode(4));
    }
    if entry.compound_last {
        entry_codes.push(FlagCode(5));
    }
    if entry.compound_middle {
        entry_codes.push(FlagCode(6));
    }
    if entry.only_in_compound {
        entry_codes.push(FlagCode(7));
    }
    if entry.compound_permit_flag {
        entry_codes.push(FlagCode(8));
    }
    if entry.forbidden_word {
        entry_codes.push(FlagCode(9));
    }
    if entry.keep_case {
        entry_codes.push(FlagCode(10));
    }
    if entry.need_affix {
        entry_codes.push(FlagCode(11));
    }
    if entry.substandard {
        entry_codes.push(FlagCode(12));
    }
    if entry.circum_fix {
        entry_codes.push(FlagCode(13));
    }

    for p in &entry.prefix {
        if !flag_codes.contains_key(p) {
            panic!("No flag code for {}", p);
        }
        let code: FlagCode = flag_codes[p];
        entry_codes.push(code);
    }

    for s in &entry.suffix {
        if !flag_codes.contains_key(s) {
            panic!("No flag code for {}", s);
        }
        let code: FlagCode = flag_codes[s];
        entry_codes.push(code);
    }

    entry_codes
}
