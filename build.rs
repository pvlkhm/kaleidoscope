extern crate lalrpop;
use std::env;

fn main() {
    env::set_var("OUT_DIR", "src");
    lalrpop::Configuration::new().use_cargo_dir_conventions().process().unwrap();
}
