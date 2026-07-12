use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display, fs, path::Path};
use toml::value::Date;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TomlDict {
    metadata: DictMetadata,
    config: Option<DictConfig>,
    prefix: Option<HashMap<String, Affix>>,
    suffix: Option<HashMap<String, Affix>>,
    replace: Option<Vec<Replace>>, // TODO
    entry: Option<Vec<DictEntry>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DictMetadata {
    // TODO make fields optional
    title: String,
    description: String,
    version: String,
    date: Date,
    authors: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DictConfig {
    encoding: Option<String>,
    additional_word_characters: Option<String>,
    #[serde(default)]
    complex_prefixes: bool,
    language_code: Option<String>,
    ignore_characters: Option<String>,
    try_characters: Option<String>,
    max_compound_suggestions: Option<u8>,
    max_n_gram_suggestions: Option<u8>,
    max_diff: Option<u8>,
    #[serde(default)]
    only_max_diff: bool,
    #[serde(default)]
    no_split_suggestions: bool,
    #[serde(default)]
    suggest_with_dots: bool,
    input_conversion: Option<Vec<Replace>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Replace {
    remove: String,
    add: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct DictEntry {
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

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct CondReplace {
    strip: Option<String>,
    add: Option<String>,
    cond: Option<String>,
    stack: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Affix {
    rules: Vec<CondReplace>,
    #[serde(default)]
    cross_product: bool,
    #[serde(default)]
    circum_fix: bool, // TODO
    #[serde(default)]
    substandard: bool, // TODO
}

#[derive(Clone, Copy)]
struct FlagCode(u16);
impl Display for FlagCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

type FlagCodeLookup = HashMap<String, FlagCode>;

fn build_flag_code_look_up(dict: &TomlDict) -> Result<FlagCodeLookup> {
    let prefixes = match &dict.prefix {
        Some(x) => x.clone(), // TODO: unnecessary cloning
        None => HashMap::new(),
    };
    let suffixes: HashMap<String, Affix> = match &dict.suffix {
        Some(x) => x.clone(), // TODO: unnecessary cloning
        None => HashMap::new(),
    };

    let k: usize = prefixes.len();

    let total_num_flags: usize = 100 + k + suffixes.len();
    if total_num_flags > 65_000 {
        let msg: &str = "Total number of flags cannot exceed 65,000";
        todo!("{}", msg); // TODO: proper error handling
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
    flag_codes.insert("{substandard_stem}".to_string(), FlagCode(12));
    flag_codes.insert("{circum_fix}".to_string(), FlagCode(13));
    flag_codes.insert("{substandard_affix}".to_string(), FlagCode(14));

    let prefix_start: u16 = 100;
    for (i, p) in (prefix_start..).zip(prefixes) {
        flag_codes.insert(p.0, FlagCode(i));
    }

    let suffix_start: u16 = (100 + k) as u16;
    for (i, p) in (suffix_start..).zip(suffixes) {
        flag_codes.insert(p.0, FlagCode(i));
    }

    Ok(flag_codes)
}

fn collect_flag_codes(entry: &DictEntry, flag_codes: &FlagCodeLookup) -> Vec<FlagCode> {
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

    if let Some(prefixes) = &entry.prefix {
        for p in prefixes {
            let code: FlagCode = flag_codes[p];
            entry_codes.push(code);
        }
    }

    if let Some(suffixes) = &entry.suffix {
        for s in suffixes {
            let code: FlagCode = flag_codes[s];
            entry_codes.push(code);
        }
    }

    entry_codes
}

pub fn load_toml_dict(path: &Path) -> Result<TomlDict> {
    let raw: String = fs::read_to_string(path)?;
    let dict: TomlDict = toml::from_str(&raw)?;
    Ok(dict)
}

pub fn build_hunspell_dictionary(out_path: &Path, dict: &TomlDict) -> Result<()> {
    let flag_codes: FlagCodeLookup = build_flag_code_look_up(dict)?;
    let base_filename: String = base_filename_from_dir(out_path)?;
    let dic_filename: &Path = &out_path.join(Path::new(&(base_filename.clone() + ".dic")));
    let dic: String = build_dic(dict, &flag_codes)?;
    fs::write(dic_filename, dic)?;
    let aff_filename: &Path = &out_path.join(Path::new(&(base_filename.clone() + ".aff")));
    let aff: String = build_aff(dict, &flag_codes)?;
    fs::write(aff_filename, aff)?;
    Ok(())
}

fn base_filename_from_dir(out_path: &Path) -> Result<String> {
    let local_dir_name: String = out_path
        .file_name()
        .context("Cannot find local folder name ({out_path:?})")?
        .to_str()
        .context("Cannot convert path to string")?
        .to_string();
    Ok(local_dir_name)
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
    let mut content: String = format!("# {} ({})\n", dict.metadata.title, dict.metadata.version);
    content += &format!("# {}\n#\n", dict.metadata.description);
    content += &format!("# {}\n#\n", dict.metadata.date); // TODO: use current datetime
    content += "# Authors:\n";
    for author in &dict.metadata.authors {
        content += &format!("#   {}\n", author);
    }
    content += "#\n# This Hunspell dictionary was created using the fix-affix tool\n";
    content += "#   https://github.com/samreynoldsmath/fix-affix\n";
    content += "#\n\n";
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
            let strip: &str = match &rule.strip {
                Some(s) => s,
                None => "0",
            };
            let add: &str = match &rule.add {
                // TODO shouldn't be optional
                Some(s) => s,
                None => "0",
            };
            let stacks: &Vec<String> = match &rule.stack {
                Some(stacks) => stacks,
                None => &vec![],
            };
            let cond: &str = match &rule.cond {
                Some(s) => s,
                None => ".",
            };
            content += &format!("{} {}   {} {}", affix_str, code, strip, add);
            if !stacks.is_empty() {
                content += "/";
                for stack_rule in stacks.iter().take(stacks.len() - 1) {
                    let stack_code: FlagCode = flag_codes[stack_rule];
                    content += &format!("{},", stack_code)
                }
                if let Some(stack_rule) = stacks.last() {
                    let stack_code: FlagCode = flag_codes[stack_rule];
                    content += &format!("{}", stack_code);
                }
            }
            content += &format!(" {}\n", cond);
        }
    }
    content
}
