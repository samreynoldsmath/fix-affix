use crate::{
    Affix, AffixRule, AffixType, CodeMap, DerivedDictData, DictConfig, FlagCode, HunspellDict,
    Replace,
};
use crate::{DATE_FMT, REPO_URL};
use anyhow::Result;
use chrono::prelude::{Local, Utc};
use std::{fs, path::Path};

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
        content += &build_replacements_string(
            &self.config.characters.try_replace,
            "REP",
            replace_formatter,
        );
        content += &build_replacements_string(
            &self.config.characters.phonetic_replace,
            "PHONE",
            replace_formatter,
        );
        content += &build_replacements_string(&self.config.characters.remap, "MAP", map_formatter);
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
        for key in affix_keys {
            let afx: Affix = affixes[key].clone();
            let num_rules: usize = afx.rules.len();
            if num_rules == 0 {
                continue;
            }
            let code: FlagCode = affix_code_map[key];
            let cross_prod: &str = match afx.cross_product {
                true => "Y",
                false => "N",
            };
            content += &format!("\n{} {} {} {}\n", affix_str, code, cross_prod, num_rules);
            for rule in &afx.rules {
                content += &build_single_affix_rule_string(
                    &self.derived.code_map,
                    code,
                    rule,
                    affix_str,
                    &affix_type,
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
    s.replace(" ", "_")
}

impl DictConfig {
    fn build_aff_preamble_string(&self) -> String {
        let mut content: String = "FLAG num\n".to_string();
        if !self.encoding.is_empty() {
            content += &format!("SET {}\n", self.encoding);
        };
        if !self.characters.additional.is_empty() {
            content += &format!("WORDCHARS {}\n", self.characters.additional);
        };
        if self.complex_prefixes {
            content += "COMPLEXPREFIXES\n"
        }
        if !self.language_code.is_empty() {
            content += &format!("LANG {}\n", self.language_code);
        }
        if !self.characters.ignore.is_empty() {
            content += &format!("IGNORE {}\n", self.characters.ignore);
        }
        if !self.characters.try_order.is_empty() {
            content += &format!("TRY {}\n", self.characters.try_order);
        }
        if !self.characters.key_groups.is_empty() {
            content += "KEY ";
            let n: usize = self.characters.key_groups.len();
            for char_group in self.characters.key_groups.iter().take(n - 1) {
                content += char_group;
                content += "|";
            }
            if let Some(char_group) = self.characters.key_groups.last() {
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
        if self.compound.check_case {
            content += "CHECKCOMPOUNDCASE\n";
        }
        if self.compound.check_duplicate {
            content += "CHECKCOMPOUNDDUP\n";
        }
        if self.compound.check_replace {
            content += "CHECKCOMPOUNDREP\n";
        }
        if self.compound.check_triple {
            content += "CHECKCOMPOUNDTRIPLE\n";
        }
        if self.compound.more_suffixes {
            content += "COMPOUNDMORESUFFIXES\n";
        }
        if self.compound.simplified_triple {
            content += "SIMPLIFIEDTRIPLE\n";
        }
        if self.compound.min_char > 0 {
            content += &format!("COMPOUNDMIN {}\n", self.compound.min_char);
        }
        if self.compound.max_word > 0 {
            content += &format!("COMPOUNDWORDMAX {}\n", self.compound.max_word);
        }
        if self.compound.max_suggestions > 0 {
            content += &format!("MAXCPDSUGS {}\n", self.compound.max_suggestions);
        }
        if !self.characters.input_conversion.is_empty() {
            content += &format!("ICONV {}\n", self.characters.input_conversion.len());
            for iconv in &self.characters.input_conversion {
                content += &format!("ICONV {} {}\n", iconv.remove, iconv.add);
            }
        }
        if !self.characters.output_conversion.is_empty() {
            content += &format!("OCONV {}\n", self.characters.output_conversion.len());
            for oconv in &self.characters.output_conversion {
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
    code_map: &CodeMap,
    code: FlagCode,
    rule: &AffixRule,
    affix_str: &str,
    affix_type: &AffixType,
) -> Result<String> {
    let strip: &str = match rule.strip.is_empty() {
        true => "0",
        false => &rule.strip,
    };
    let add: &str = match rule.add.is_empty() {
        true => "0",
        false => &rule.add,
    };
    let cond: &str = match rule.cond.is_empty() {
        true => ".",
        false => &rule.cond,
    };
    let mut content: String = format!("{} {}   {} {}", affix_str, code, strip, add);
    content += &build_affix_flag_string(code_map, rule, affix_type)?;
    content += &format!(" {}\n", cond);
    Ok(content)
}

fn build_affix_flag_string(
    code_map: &CodeMap,
    rule: &AffixRule,
    affix_type: &AffixType,
) -> Result<String> {
    let flag_codes: Vec<FlagCode> = rule.collect_flag_codes(code_map, affix_type)?;
    if flag_codes.is_empty() {
        return Ok("".to_string());
    }
    let mut content: String = "/".to_string();
    for code in flag_codes.iter().take(flag_codes.len() - 1) {
        content += &format!("{},", code)
    }
    if let Some(code) = flag_codes.last() {
        content += &format!("{}", code);
    }
    Ok(content)
}
