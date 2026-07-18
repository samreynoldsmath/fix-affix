use crate::HunspellDict;
use anyhow::Result;
use std::path::Path;

impl HunspellDict {
    /// Loads dictionary from a TOML formatted string
    pub fn load_from_toml_string(data: &str) -> Result<HunspellDict> {
        let mut dict: HunspellDict = toml::from_str(data)?;
        dict.compute_derived_data()?;
        Ok(dict)
    }

    /// Loads dictionary from a TOML file
    pub fn load_from_toml_file(path: &Path) -> Result<HunspellDict> {
        let data: String = std::fs::read_to_string(path)?;
        Self::load_from_toml_string(&data)
    }
}
