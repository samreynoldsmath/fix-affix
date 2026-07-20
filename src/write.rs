use crate::{Affix, CondReplace, DerivedDictData, DictConfig, FlagCode, HunspellDict, Replace};
use crate::{DATE_FMT, REPO_URL};
use anyhow::{Error, Result};
use chrono::prelude::{Local, Utc};
use std::collections::HashMap;
use std::{fs, path::Path};

enum AffixType {
    Prefix,
    Suffix,
}

impl HunspellDict {
    /// Writes the .dic file
    pub fn write_dic_file(&self, dic_file: &Path) -> Result<()> {
        let dic: String = self.build_dic_string()?;
        fs::write(dic_file, dic)?;
        Ok(())
    }

    /// Writes the .aff file
    pub fn write_aff_file(&self, aff_file: &Path) -> Result<()> {
        let aff: String = self.build_aff_string()?;
        fs::write(aff_file, aff)?;
        Ok(())
    }

    /// Returns a string containing the contents of the .dic file
    pub fn build_dic_string(&self) -> Result<String> {
        let mut content: String = format!("{}\n", self.entry.len());
        for word in &self.entry {
            content += &word.stem;
            let entry_codes: Vec<FlagCode> = word.collect_flag_codes(&self.derived.code_map)?;
            if entry_codes.is_empty() {
                content += "\n";
                continue;
            }
            content += "/";
            for code in entry_codes.iter().take(entry_codes.len() - 1) {
                content += &format!("{},", code);
            }
            if let Some(code) = entry_codes.last() {
                content += &format!("{}\n", code);
            }
        }
        Ok(content)
    }

    /// Returns a string containing the contents of the .aff file
    pub fn build_aff_string(&self) -> Result<String> {
        let mut content: String = self.build_aff_header();
        content += &self.config.build_aff_preamble_string();
        content += &self.derived.build_flag_keys_string();
        content += &self.build_affix_rules_string(AffixType::Prefix)?;
        content += &self.build_affix_rules_string(AffixType::Suffix)?;
        content += &build_replacements_string(&self.config.replace, "REP", replace_formatter);
        content +=
            &build_replacements_string(&self.config.phonetic_replace, "PHONE", replace_formatter);
        content += &build_replacements_string(&self.config.map_characters, "MAP", map_formatter);
        Ok(content)
    }

    fn build_aff_header(&self) -> String {
        let now: String = Local::now().format(DATE_FMT).to_string();
        let utc: String = Utc::now().format(DATE_FMT).to_string();

        let mut content: String =
            format!("# {} ({})\n", self.metadata.title, self.metadata.version);
        content += &format!("# {}\n#\n", self.metadata.description);
        content += &format!("# {} (UTC {})\n#\n", now, utc);
        content += "# Authors:\n";
        for author in &self.metadata.authors {
            content += &format!("#   {}\n", author);
        }

        content += "#\n# This Hunspell dictionary was created using ";
        content += &format!("fix-affix v{}\n", clap::crate_version!());
        content += &format!("#   {}\n\n", REPO_URL);
        content
    }

    fn build_affix_rules_string(&self, affix_type: AffixType) -> Result<String> {
        let mut content: String = "".to_string();
        let (affix_keys, affixes, affix_str, affix_code_map) = match affix_type {
            AffixType::Prefix => (
                &self.derived.sorted_prefix_keys,
                &self.prefix,
                "PFX",
                &self.derived.code_map.pfx_map,
            ),
            AffixType::Suffix => (
                &self.derived.sorted_suffix_keys,
                &self.suffix,
                "SFX",
                &self.derived.code_map.sfx_map,
            ),
        };
        for k in affix_keys {
            let afx: Affix = affixes[k].clone();
            let num_rules: usize = afx.rules.len();
            if num_rules == 0 {
                continue;
            }
            let code: FlagCode = affix_code_map[k];
            let cross_prod: &str = match afx.cross_product {
                true => "Y",
                false => "N",
            };
            content += &format!("\n{} {} {} {}\n", affix_str, code, cross_prod, num_rules);
            for rule in &afx.rules {
                content += &build_single_affix_rule_string(
                    rule,
                    affix_code_map,
                    affix_str,
                    code,
                    afx.substandard,
                    afx.circumfix,
                )?;
            }
        }
        Ok(content)
    }
}

fn build_replacements_string(
    reps: &Vec<Replace>,
    keyword: &str,
    formatter: fn(&str, &Replace) -> String,
) -> String {
    if reps.is_empty() {
        return "".to_string();
    }
    let num_reps: usize = reps.len();
    let mut content: String = format!("\n{} {}\n", keyword, num_reps);
    for rep in reps {
        content += &formatter(keyword, rep);
    }
    content
}

fn map_formatter(keyword: &str, rep: &Replace) -> String {
    let rm: String = wrap_string_in_paren_if_len_not_one(&rep.remove);
    let add: String = wrap_string_in_paren_if_len_not_one(&rep.add);
    format!("{} {}{}\n", keyword, rm, add)
}

fn wrap_string_in_paren_if_len_not_one(s: &str) -> String {
    match s.chars().count() {
        1 => s.to_string(),
        _ => format!("({})", s),
    }
}

fn replace_formatter(keyword: &str, rep: &Replace) -> String {
    let rm: String = replace_space_with_underscore(&rep.remove);
    let add: String = replace_space_with_underscore(&rep.add);
    format!("{} {} {}\n", keyword, rm, add)
}

fn replace_space_with_underscore(s: &str) -> String {
    match s.chars().count() {
        1 => s.to_string(),
        _ => format!("({})", s),
    }
}

impl DictConfig {
    fn build_aff_preamble_string(&self) -> String {
        let mut content: String = "FLAG num\n".to_string();
        if !self.encoding.is_empty() {
            content += &format!("SET {}\n", self.encoding);
        };
        if !self.additional_word_characters.is_empty() {
            content += &format!("WORDCHARS {}\n", self.additional_word_characters);
        };
        if self.complex_prefixes {
            content += "COMPLEXPREFIXES\n"
        }
        if !self.language_code.is_empty() {
            content += &format!("LANG {}\n", self.language_code);
        }
        if !self.ignore_characters.is_empty() {
            content += &format!("IGNORE {}\n", self.ignore_characters);
        }
        if !self.try_characters.is_empty() {
            content += &format!("TRY {}\n", self.try_characters);
        }
        if !self.key_characters.is_empty() {
            content += "KEY ";
            let n: usize = self.key_characters.len();
            for char_group in self.key_characters.iter().take(n - 1) {
                content += char_group;
                content += "|";
            }
            if let Some(char_group) = self.key_characters.last() {
                content += char_group;
            }
            content += "\n";
        }
        if self.max_n_gram_suggestions > 0 {
            content += &format!("MAXNGRAMSUGS {}\n", self.max_n_gram_suggestions);
        }
        if self.max_diff > 0 {
            content += &format!("MAXDIFF {}\n", self.max_diff);
        }
        if self.only_max_diff {
            content += "ONLYMAXDIFF\n";
        }
        if self.no_split_suggestions {
            content += "NOSPLITSUGS\n";
        }
        if self.suggest_with_dots {
            content += "SUGSWITHDOTS\n";
        }
        if self.forbid_warn {
            content += "FORBIDWARN\n";
        }
        if self.full_strip {
            content += "FULLSTRIP\n";
        }
        if self.check_sharps {
            content += "CHECKSHARPS\n";
        }
        if !self.input_conversion.is_empty() {
            content += &format!("ICONV {}\n", self.input_conversion.len());
            for iconv in &self.input_conversion {
                content += &format!("ICONV {} {}\n", iconv.remove, iconv.add);
            }
        }
        if !self.output_conversion.is_empty() {
            content += &format!("OCONV {}\n", self.output_conversion.len());
            for oconv in &self.output_conversion {
                content += &format!("OCONV {} {}\n", oconv.remove, oconv.add);
            }
        }
        content
    }
}

impl DerivedDictData {
    fn build_flag_keys_string(&self) -> String {
        let mut content: String = "".to_string();
        for (key, code) in &self.code_map.cfg_map {
            if !self.used_flags.contains(code) {
                continue;
            }
            content += &format!("{} {}\n", key, code.0);
        }
        content
    }
}

fn build_single_affix_rule_string(
    rule: &CondReplace,
    affix_code_map: &HashMap<String, FlagCode>,
    affix_str: &str,
    code: FlagCode,
    substandard: bool,
    circumfix: bool,
) -> Result<String> {
    let strip: &str = match &rule.strip {
        Some(s) => s,
        None => "0",
    };
    let cond: &str = match &rule.cond {
        Some(s) => s,
        None => ".",
    };
    let mut affix_flags: Vec<String> = match &rule.stack {
        Some(stacks) => stacks.clone(),
        None => vec![],
    };
    if substandard {
        affix_flags.push("substandard".to_string());
    }
    if circumfix {
        affix_flags.push("circumfix".to_string());
    }
    affix_flags.sort();
    let mut content: String = format!("{} {}   {} {}", affix_str, code, strip, rule.add);
    content += &build_affix_flag_string(&affix_flags, affix_code_map)?;
    content += &format!(" {}\n", cond);
    Ok(content)
}

fn build_affix_flag_string(
    affix_flags: &[String],
    affix_code_map: &HashMap<String, FlagCode>,
) -> Result<String> {
    if affix_flags.is_empty() {
        return Ok("".to_string());
    }
    let mut content: String = "/".to_string();
    for flag in affix_flags.iter().take(affix_flags.len() - 1) {
        if !affix_code_map.contains_key(flag) {
            let e: Error = Error::msg(format!("No flag code for {}", flag));
            return Err(e);
        }
        let code: FlagCode = affix_code_map[flag];
        content += &format!("{},", code)
    }
    if let Some(flag) = affix_flags.last() {
        if !affix_code_map.contains_key(flag) {
            let e: Error = Error::msg(format!("No flag code for {}", flag));
            return Err(e);
        }
        let code: FlagCode = affix_code_map[flag];
        content += &format!("{}", code);
    }
    Ok(content)
}
