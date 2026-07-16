use crate::{
    Affix, FlagCode, HunspellDict, VERSION,
    process::get_used_flags,
    read::{CondReplace, DictConfig, Replace},
};
use anyhow::Result;
use chrono::prelude::{Local, Utc};
use std::collections::HashMap;
use std::{fs, path::Path};

const DATE_FMT: &str = "%Y-%m-%d %H:%M";
const REPO_URL: &str = "https://github.com/samreynoldsmath/fix-affix";

enum AffixType {
    Prefix,
    Suffix,
}

impl HunspellDict {
    pub fn write_dic_file(&self, dic_file: &Path) -> Result<()> {
        let dic: String = self.build_dic_string();
        fs::write(dic_file, dic)?;
        Ok(())
    }

    pub fn write_aff_file(&self, aff_file: &Path) -> Result<()> {
        let aff: String = self.build_aff_string();
        fs::write(aff_file, aff)?;
        Ok(())
    }

    pub fn build_dic_string(&self) -> String {
        let mut content: String = format!("{}\n", self.entry.len());
        for word in &self.entry {
            content += &word.stem;
            let entry_codes: Vec<FlagCode> = word.collect_flag_codes(&self.derived.flag_codes);
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
        content
    }

    pub fn build_aff_string(&self) -> String {
        let used_flags: Vec<FlagCode> = get_used_flags(&self.entry, &self.derived.flag_codes);

        let mut content: String = self.build_aff_header();
        content += &self.config.build_aff_preamble_string();
        content += &build_flag_keys_string(used_flags);
        content += &self.build_affix_rules_string(AffixType::Prefix);
        content += &self.build_affix_rules_string(AffixType::Suffix);
        content += &build_replacements_string(&self.replace);
        content
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
        content += &format!("fix-affix v{}\n", VERSION);
        content += &format!("#   {}\n\n", REPO_URL);
        content
    }

    fn build_affix_rules_string(&self, affix_type: AffixType) -> String {
        let mut content: String = "".to_string();
        let (affix_keys, affixes, affix_str) = match affix_type {
            AffixType::Prefix => (&self.derived.sorted_prefix, &self.prefix, "PFX"),
            AffixType::Suffix => (&self.derived.sorted_suffix, &self.suffix, "SFX"),
        };
        for k in affix_keys {
            let afx: Affix = affixes[k].clone();
            let num_rules: usize = afx.rules.len();
            if num_rules == 0 {
                continue;
            }
            let code: FlagCode = self.derived.flag_codes[k];
            let cross_prod: &str = match afx.cross_product {
                true => "Y",
                false => "N",
            };
            content += &format!("\n{} {} {} {}\n", affix_str, code, cross_prod, num_rules);
            for rule in &afx.rules {
                content += &build_single_affix_rule_string(
                    rule,
                    &self.derived.flag_codes,
                    affix_str,
                    code,
                    afx.substandard,
                    afx.circum_fix,
                );
            }
        }
        content
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
        if self.max_compound_suggestions > 0 {
            content += &format!("MAXCPDSUGS {}\n", self.max_compound_suggestions);
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
        if !self.input_conversion.is_empty() {
            content += &format!("ICONV {}\n", self.input_conversion.len());
            for iconv in &self.input_conversion {
                content += &format!("ICONV {} {}\n", iconv.remove, iconv.add);
            }
        }
        content
    }
}

fn build_flag_keys_string(used_flags: Vec<FlagCode>) -> String {
    let mut content: String = "".to_string();
    for code in used_flags {
        content += match code {
            FlagCode(0) => "NOSUGGEST 0\n",
            FlagCode(1) => "WARN 1\n",
            FlagCode(2) => "FORBIDWARN 2\n",
            FlagCode(3) => "COMPOUNDFLAG 3\n",
            FlagCode(4) => "COMPOUNDBEGIN 4\n",
            FlagCode(5) => "COMPOUNDLAST 5\n",
            FlagCode(6) => "COMPOUNDMIDDLE 6\n",
            FlagCode(7) => "ONLYINCOMPOUND 7\n",
            FlagCode(8) => "COMPOUNDPERMITFLAG 8\n",
            FlagCode(9) => "FORBIDDENWORD 9\n",
            FlagCode(10) => "KEEPCASE 10\n",
            FlagCode(11) => "NEEDAFFIX 11\n",
            FlagCode(12) => "SUBSTANDARD 12\n",
            FlagCode(13) => "CIRCUMFIX 13\n",
            FlagCode(x) => panic!("Unknown FlagCode({})", x),
        }
    }
    content
}

fn build_single_affix_rule_string(
    rule: &CondReplace,
    flag_codes: &HashMap<String, FlagCode>,
    affix_str: &str,
    code: FlagCode,
    substandard: bool,
    circum_fix: bool,
) -> String {
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
    if circum_fix {
        affix_flags.push("circum_fix".to_string());
    }
    affix_flags.sort();
    let mut content: String = format!("{} {}   {} {}", affix_str, code, strip, &rule.add);
    content += &build_affix_flag_string(&affix_flags, flag_codes);
    content += &format!(" {}\n", cond);
    content
}

fn build_affix_flag_string(
    affix_flags: &[String],
    flag_codes: &HashMap<String, FlagCode>,
) -> String {
    if affix_flags.is_empty() {
        return "".to_string();
    }
    let mut content: String = "/".to_string();
    for flag in affix_flags.iter().take(affix_flags.len() - 1) {
        if !flag_codes.contains_key(flag) {
            panic!("No flag code for {}", flag);
        }
        let code: FlagCode = flag_codes[flag];
        content += &format!("{},", code)
    }
    if let Some(flag) = affix_flags.last() {
        if !flag_codes.contains_key(flag) {
            panic!("No flag code for {}", flag);
        }
        let code: FlagCode = flag_codes[flag];
        content += &format!("{}", code);
    }
    content
}

fn build_replacements_string(reps: &Vec<Replace>) -> String {
    if reps.is_empty() {
        return "".to_string();
    }
    let num_reps: usize = reps.len();
    let mut content: String = format!("\nREP {}\n", num_reps);
    for r in reps {
        let rm: String = r.remove.replace(" ", "_");
        let add: String = r.add.replace(" ", "_");
        content += &format!("REP {} {}\n", rm, add);
    }
    content
}
