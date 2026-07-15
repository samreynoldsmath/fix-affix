use crate::{
    Affix, DictEntry, FlagCode, FlagCodeLookup, TomlDict, VERSION, build_flag_code_look_up,
    collect_flag_codes, get_sorted_affixes, read::CondReplace,
};
use anyhow::Result;
use chrono::prelude::{Local, Utc};
use std::{collections::HashMap, fs, path::Path};

const DATE_FMT: &str = "%Y-%m-%d %H:%M";
const REPO_URL: &str = "https://github.com/samreynoldsmath/fix-affix";

pub fn build_hunspell_dictionary(dict: &TomlDict, aff_file: &Path, dic_file: &Path) -> Result<()> {
    let flag_codes: FlagCodeLookup = build_flag_code_look_up(dict)?;
    let dic: String = build_dic(dict, &flag_codes)?;
    let aff: String = build_aff(dict, &flag_codes)?;
    fs::write(dic_file, dic)?;
    fs::write(aff_file, aff)?;
    Ok(())
}

fn build_dic(dict: &TomlDict, flag_codes: &FlagCodeLookup) -> Result<String> {
    let entries: Vec<DictEntry> = match &dict.entry {
        Some(x) => x.to_vec(),
        None => vec![],
    };
    let mut content: String = format!("{}\n", entries.len());
    for entry in entries {
        content += &entry.stem;
        let entry_codes: Vec<FlagCode> = collect_flag_codes(&entry, flag_codes);
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

fn build_aff(dict: &TomlDict, flag_codes: &FlagCodeLookup) -> Result<String> {
    let mut content: String = write_aff_header(dict);
    content += &write_aff_preamble(dict);
    content += &write_flag_keys();
    let prefixes = match &dict.prefix {
        Some(x) => x.clone(), // TODO: unnecessary cloning
        None => HashMap::new(),
    };
    let suffixes: HashMap<String, Affix> = match &dict.suffix {
        Some(x) => x.clone(), // TODO: unnecessary cloning
        None => HashMap::new(),
    };
    content += &write_affix_rules(&prefixes, "PFX", flag_codes);
    content += &write_affix_rules(&suffixes, "SFX", flag_codes);
    Ok(content)
}

fn write_aff_header(dict: &TomlDict) -> String {
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

fn write_aff_preamble(dict: &TomlDict) -> String {
    let config = match &dict.config {
        Some(config) => config,
        _ => return "".to_string(),
    };
    let mut content: String = "FLAG num\n".to_string();
    if let Some(encoding) = &config.encoding {
        content += &format!("SET {}\n", encoding);
    };
    if let Some(word_char) = &config.additional_word_characters {
        content += &format!("WORDCHARS {}\n", word_char);
    };
    if config.complex_prefixes {
        content += "COMPLEXPREFIXES\n"
    }
    if let Some(language_code) = &config.language_code {
        content += &format!("LANG {}\n", language_code);
    }
    if let Some(ignore_characters) = &config.ignore_characters {
        content += &format!("IGNORE {}\n", ignore_characters);
    }
    if let Some(try_characters) = &config.try_characters {
        content += &format!("TRY {}\n", try_characters);
    }
    if let Some(max_compound_suggestions) = &config.max_compound_suggestions {
        content += &format!("MAXCPDSUGS {}\n", max_compound_suggestions);
    }
    if let Some(max_n_gram_suggestions) = &config.max_n_gram_suggestions {
        content += &format!("MAXNGRAMSUGS {}\n", max_n_gram_suggestions);
    }
    if let Some(max_diff) = &config.max_diff {
        content += &format!("MAXDIFF {}\n", max_diff);
    }
    if config.only_max_diff {
        content += "ONLYMAXDIFF\n";
    }
    if config.no_split_suggestions {
        content += "NOSPLITSUGS\n";
    }
    if config.suggest_with_dots {
        content += "SUGSWITHDOTS\n";
    }
    if let Some(input_conversion) = &config.input_conversion
        && !input_conversion.is_empty()
    {
        content += &format!("ICONV {}\n", input_conversion.len());
        for iconv in input_conversion {
            content += &format!("ICONV {} {}\n", iconv.remove, iconv.add);
        }
    }
    content
}

fn write_flag_keys() -> String {
    "NOSUGGEST 0
WARN 1
FORBIDWARN 2
COMPOUNDFLAG 3
COMPOUNDBEGIN 4
COMPOUNDLAST 5
COMPOUNDMIDDLE 6
ONLYINCOMPOUND 7
COMPOUNDPERMITFLAG 8
FORBIDDENWORD 9
KEEPCASE 10
NEEDAFFIX 11
SUBSTANDARD 12
CIRCUMFIX 13
"
    .to_string()
}

fn write_affix_rules(
    affixes: &HashMap<String, Affix>,
    affix_str: &str,
    flag_codes: &FlagCodeLookup,
) -> String {
    let mut content: String = "".to_string();
    // TODO: vectorization should only happen once
    let vec_affix: Vec<(&String, &Affix)> = get_sorted_affixes(affixes);
    for (a, afx) in vec_affix {
        let num_rules: usize = afx.rules.len();
        if num_rules == 0 {
            continue;
        }
        if !flag_codes.contains_key(a) {
            panic!("No flag code for {}", a);
        }
        let code: FlagCode = flag_codes[a];
        let cross_prod: &str = match afx.cross_product {
            true => "Y",
            false => "N",
        };
        content += &format!("\n{} {} {} {}\n", affix_str, code, cross_prod, num_rules);
        for rule in &afx.rules {
            content += &write_rule(rule, flag_codes, affix_str, code);
        }
    }
    content
}

fn write_rule(
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
    content += &write_stacks(stacks, flag_codes);
    content += &format!(" {}\n", cond);
    content
}

fn write_stacks(stacks: &[String], flag_codes: &FlagCodeLookup) -> String {
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
