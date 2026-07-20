use std::process::exit;
use gsr::env::configuration::Configuration;
use gsr::model::catalog::Catalog;

pub fn main() {
    let config = match Configuration::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };
    if let Ok(catalog) = Catalog::from_file(&config.catalog_filepath) {
        println!("{}", catalog.s_expression());
    } else {
        println!("can't open file {}", config.catalog_filepath);
    } 
}
