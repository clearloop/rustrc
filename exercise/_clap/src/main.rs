extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    App::new("\n_clappp\n")
        .version("1.0")
        .about("Does great things")
        .author("udtrokia")
        .get_matches();
}
