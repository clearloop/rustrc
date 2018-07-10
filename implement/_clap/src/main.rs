#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();
    
    // Same as previous examples...
    if let Some(_matches) = _matches.subcommand_matches("test") {
        if _matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...")
        }
    }
}
