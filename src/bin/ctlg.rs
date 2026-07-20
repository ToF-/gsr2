use clap::Subcommand;
use clap::Parser;
use std::process::exit;
use gsr::env::configuration::Configuration;
use gsr::model::catalog::Catalog;

#[derive(Parser, Clone, Debug, PartialEq)]
/// Catalog
#[command(
    about("category catalog for gsr"),
    author("ToF"),
    version,
    infer_long_args = true,
    infer_subcommands = true,
    help_template(
        "\
{before-help}{name} {version} {about} by {author-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"
    )
)]

/// list, add or remove categories
pub struct Command {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Commands {
    /// add <SUB_CATEGORY> to <CATEGORY>
    Add {
        #[arg(short,long, value_name = "SUB_CATEGORY")]
        sub_category: String,

        #[arg(short, long)]
        category: String,
    },
    /// list all categories (default)
    List,
    /// remove <CATEGORY>
    Remove {
        #[arg(short, long)]
        category: String,
    }
}

pub fn list(catalog: &Catalog) {
    println!("{}", catalog.root().format_at_level(0));
}

pub fn main() {
    let config = match Configuration::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };
    if let Ok(mut catalog) = Catalog::from_file(&config.catalog_filepath) {
        let command = Command::parse();
        if let Some(command) = command.commands {
            match command {
                Commands::List => list(&catalog),
                Commands::Add { sub_category, category } => {
                    match catalog.add_sub_category(&sub_category, &category) {
                        Ok(_) => {
                            let new_catalog = catalog.clone();
                            match new_catalog.save_to_file(&config.catalog_filepath) {
                                Ok(_) => {
                                    println!("adding sub category {} to category {}", sub_category, category);
                                },
                                Err(err) => eprintln!("error: {}", err),
                            }
                        },
                        Err(err) => eprintln!("error: {}", err),
                    }
                },
                Commands::Remove { category } => {
                    match catalog.remove_category(&category) {
                        Ok(_) => {
                            let new_catalog = catalog.clone();
                            match new_catalog.save_to_file(&config.catalog_filepath) {
                                Ok(_) => println!("removing category {}", category),
                                Err(err) => eprintln!("error: {}", err),
                            };
                        },
                        Err(err) => eprintln!("error: {}", err),
                    }
                },
            }
        } else {
            list(&catalog)
        }

    } else {
        println!("can't open file {}", config.catalog_filepath);
    } 
}
