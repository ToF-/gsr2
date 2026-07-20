#[path = "../env.rs"]
mod env;
use std::process::exit;


pub fn main() {
    let config = match Configuration::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };
    println!("foo");
}
