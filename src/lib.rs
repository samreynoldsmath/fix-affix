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
    replace: Option<Vec<Replace>>,
    entry: Option<Vec<DictEntry>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DictMetadata {
    title: String,
    description: String,
    version: String,
    date: Date,
    authors: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DictConfig {
    encoding: Option<String>,
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

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct CondReplace {
    strip: Option<String>,
    add: Option<String>,
    cond: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
struct Affix {
    rules: Vec<CondReplace>,
    #[serde(default)]
    cross_product: bool,
    #[serde(default)]
    circum_fix: bool,
    #[serde(default)]
    substandard: bool,
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
    let mut content: String = aff_header(dict);

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

fn aff_header(dict: &TomlDict) -> String {
    let mut content: String = format!("# {} ({})\n", dict.metadata.title, dict.metadata.version);
    content += &format!("# {}\n#\n", dict.metadata.description);
    content += &format!("# {}\n#\n", dict.metadata.date);
    content += "# Authors:\n";
    for author in &dict.metadata.authors {
        content += &format!("#   {}\n", author);
    }
    content += "#\n# This Hunspell dictionary was created using the fix-affix tool\n";
    content += "#   https://github.com/samreynoldsmath/fix-affix\n";

    content
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
            let cond: &str = match &rule.cond {
                Some(s) => s,
                None => ".",
            };
            content += &format!("{} {}   {} {} {}\n", affix_str, code, strip, add, cond);
        }
    }
    content
}
