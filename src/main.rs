use fix_affix::{ContextManager, HunspellDict};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let ctx: ContextManager = ContextManager::new(args);

    let dict: HunspellDict = match HunspellDict::load_from_toml_file(&ctx.toml_file) {
        Ok(data) => data,
        Err(e) => panic!("TOML dictionary not loaded ({:?}): {}", &ctx.toml_file, e),
    };

    if let Err(e) = dict.write_dic_file(&ctx.dic_file) {
        panic!("Failed to build Hunspell dic: {}", e)
    };

    if let Err(e) = dict.write_aff_file(&ctx.aff_file) {
        panic!("Failed to build Hunspell aff: {}", e)
    };
}
