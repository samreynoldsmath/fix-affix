use clap::Parser;
use fix_affix::HunspellDict;
use std::path::PathBuf;

fn main() {
    let args: Args = Args::parse();

    let aff_file: PathBuf = args.toml_file.with_extension("aff");
    let dic_file: PathBuf = args.toml_file.with_extension("dic");

    let dict: HunspellDict = match HunspellDict::load_from_toml_file(&args.toml_file) {
        Ok(data) => data,
        Err(e) => {
            println!(
                "Failed to load TOML dictionary ({:?}): {}",
                args.toml_file, e
            );
            return;
        }
    };

    if let Err(e) = dict.write_dic_file(&dic_file) {
        println!("Failed to build Hunspell dic: {}", e);
        return;
    };

    if let Err(e) = dict.write_aff_file(&aff_file) {
        println!("Failed to build Hunspell aff: {}", e)
    };
}

#[derive(Parser)]
#[command(arg_required_else_help = true, version, about)]
struct Args {
    #[arg(index = 1)]
    toml_file: PathBuf,
}
