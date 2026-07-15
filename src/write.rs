use crate::{
    Affix, DictEntry, FlagCode, FlagCodeLookup, TomlDict, VERSION, build_flag_code_look_up,
    collect_flag_codes, get_sorted_affixes,
    process::get_used_flags,
    read::{CondReplace, Replace},
};
use anyhow::Result;
use chrono::prelude::{Local, Utc};
use std::{fs, path::Path};

const DATE_FMT: &str = "%Y-%m-%d %H:%M";
const REPO_URL: &str = "https://github.com/samreynoldsmath/fix-affix";

pub fn build_hunspell_dictionary(dict: &TomlDict, aff_file: &Path, dic_file: &Path) -> Result<()> {
    let prefixes: Vec<(&String, &Affix)> = get_sorted_affixes(&dict.prefix);
    let suffixes: Vec<(&String, &Affix)> = get_sorted_affixes(&dict.suffix);
    let flag_codes: FlagCodeLookup = build_flag_code_look_up(&prefixes, &suffixes)?;
    let dic: String = build_dic_string(&dict.entry, &flag_codes);
    let aff: String = build_aff_string(prefixes, suffixes, dict, &flag_codes);
    fs::write(dic_file, dic)?;
    fs::write(aff_file, aff)?;
    Ok(())
}

fn build_dic_string(entries: &Vec<DictEntry>, flag_codes: &FlagCodeLookup) -> String {
    let mut content: String = format!("{}\n", entries.len());
    for entry in entries {
        content += &entry.stem;
        let entry_codes: Vec<FlagCode> = collect_flag_codes(entry, flag_codes);
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

fn build_aff_string(
    prefixes: Vec<(&String, &Affix)>,
    suffixes: Vec<(&String, &Affix)>,
    dict: &TomlDict,
    flag_codes: &FlagCodeLookup,
) -> String {
    let used_flags: Vec<FlagCode> = get_used_flags(&dict.entry, flag_codes);

    let mut content: String = build_aff_header(dict);
    content += &build_aff_preamble_string(dict);
    content += &build_flag_keys_string(used_flags);
    content += &build_affix_rules_string(prefixes, "PFX", flag_codes);
    content += &build_affix_rules_string(suffixes, "SFX", flag_codes);
    content += &build_replacements_string(dict.replace.clone());
    content
}

fn build_aff_header(dict: &TomlDict) -> String {
    let now: String = Local::now().format(DATE_FMT).to_string();
    let utc: String = Utc::now().format(DATE_FMT).to_string();

    let mut content: String = format!("# {} ({})\n", dict.metadata.title, dict.metadata.version);
    content += &format!("# {}\n#\n", dict.metadata.description);
    content += &format!("# {} (UTC {})\n#\n", now, utc);
    content += "# Authors:\n";
    for author in &dict.metadata.authors {
        content += &format!("#   {}\n", author);
    }

    content += "#\n# This Hunspell dictionary was created using ";
    content += &format!("fix-affix v{}\n", VERSION);
    content += &format!("#   {}\n\n", REPO_URL);
    content
}

fn build_aff_preamble_string(dict: &TomlDict) -> String {
    let mut content: String = "FLAG num\n".to_string();
    if !dict.config.encoding.is_empty() {
        content += &format!("SET {}\n", dict.config.encoding);
    };
    if !dict.config.additional_word_characters.is_empty() {
        content += &format!("WORDCHARS {}\n", dict.config.additional_word_characters);
    };
    if dict.config.complex_prefixes {
        content += "COMPLEXPREFIXES\n"
    }
    if !dict.config.language_code.is_empty() {
        content += &format!("LANG {}\n", dict.config.language_code);
    }
    if !dict.config.ignore_characters.is_empty() {
        content += &format!("IGNORE {}\n", dict.config.ignore_characters);
    }
    if !dict.config.try_characters.is_empty() {
        content += &format!("TRY {}\n", dict.config.try_characters);
    }
    if dict.config.max_compound_suggestions > 0 {
        content += &format!("MAXCPDSUGS {}\n", dict.config.max_compound_suggestions);
    }
    if dict.config.max_n_gram_suggestions > 0 {
        content += &format!("MAXNGRAMSUGS {}\n", dict.config.max_n_gram_suggestions);
    }
    if dict.config.max_diff > 0 {
        content += &format!("MAXDIFF {}\n", dict.config.max_diff);
    }
    if dict.config.only_max_diff {
        content += "ONLYMAXDIFF\n";
    }
    if dict.config.no_split_suggestions {
        content += "NOSPLITSUGS\n";
    }
    if dict.config.suggest_with_dots {
        content += "SUGSWITHDOTS\n";
    }
    if !dict.config.input_conversion.is_empty() {
        content += &format!("ICONV {}\n", dict.config.input_conversion.len());
        for iconv in &dict.config.input_conversion {
            content += &format!("ICONV {} {}\n", iconv.remove, iconv.add);
        }
    }
    content
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

fn build_affix_rules_string(
    affixes: Vec<(&String, &Affix)>,
    affix_str: &str,
    flag_codes: &FlagCodeLookup,
) -> String {
    let mut content: String = "".to_string();
    for (a, afx) in affixes {
        let num_rules: usize = afx.rules.len();
        if num_rules == 0 {
            continue;
        }
        let code: FlagCode = flag_codes[a];
        let cross_prod: &str = match afx.cross_product {
            true => "Y",
            false => "N",
        };
        content += &format!("\n{} {} {} {}\n", affix_str, code, cross_prod, num_rules);
        for rule in &afx.rules {
            content += &build_single_affix_rule_string(rule, flag_codes, affix_str, code);
        }
    }
    content
}

fn build_single_affix_rule_string(
    rule: &CondReplace,
    flag_codes: &FlagCodeLookup,
    affix_str: &str,
    code: FlagCode,
) -> String {
    let strip: &str = match &rule.strip {
        Some(s) => s,
        None => "0",
    };
    let cond: &str = match &rule.cond {
        Some(s) => s,
        None => ".",
    };
    let stacks: &Vec<String> = match &rule.stack {
        Some(stacks) => stacks,
        None => &vec![],
    };
    let mut content: String = format!("{} {}   {} {}", affix_str, code, strip, &rule.add);
    content += &build_stacks_string(stacks, flag_codes);
    content += &format!(" {}\n", cond);
    content
}

fn build_stacks_string(stacks: &[String], flag_codes: &FlagCodeLookup) -> String {
    if stacks.is_empty() {
        return "".to_string();
    }
    let mut content = "/".to_string();
    for stack_rule in stacks.iter().take(stacks.len() - 1) {
        if !flag_codes.contains_key(stack_rule) {
            panic!("No flag code for {}", stack_rule);
        }
        let stack_code: FlagCode = flag_codes[stack_rule];
        content += &format!("{},", stack_code)
    }
    if let Some(stack_rule) = stacks.last() {
        if !flag_codes.contains_key(stack_rule) {
            panic!("No flag code for {}", stack_rule);
        }
        let stack_code: FlagCode = flag_codes[stack_rule];
        content += &format!("{}", stack_code);
    }
    content
}

fn build_replacements_string(reps: Vec<Replace>) -> String {
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
