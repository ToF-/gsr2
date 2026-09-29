use clap::Parser;
use clap::Subcommand;
use gsr::env::configuration::Configuration;
use gsr::file::database::Database;
use gsr::model::catalog::Catalog;
use std::process::exit;

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
        #[arg(short, long, value_name = "SUB_CATEGORY")]
        sub_category: String,

        #[arg(short, long, value_name = "CATEGORY")]
        category: String,
    },
    /// import <FILE>
    Import {
        #[arg(short, long, value_name = "FILE")]
        file: String,
    },
    /// list all categories (default)
    List,
    /// move <SUB_CATEGORY> under <CATEGORY>
    Move {
        #[arg(short, long, value_name = "SUB_CATEGORY")]
        sub_category: String,

        #[arg(short, long, value_name = "CATEGORY")]
        category: String,
    },
    /// remove <CATEGORY> [--force]
    Remove {
        #[arg(short, long)]
        category: String,

        #[arg(short, long, default_value = "false")]
        force: bool,
    },
}

pub fn list(catalog: &Catalog) {
    println!("{}", catalog.root_category().format_at_level(0, true));
}

fn save_catalog(catalog: &Catalog, database: &Database) {
    match database.rusqlite_update_catalog(&catalog.to_sexp()) {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }
}

pub fn main() {
    let config = match Configuration::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };
    let database = Database::from_connection(&config.database_file, false).unwrap();
    let catalog_result = match database.retrieve_catalog() {
        Ok(s_expression) => Catalog::from_s_expression(&s_expression),
        Err(e) => Err(e),
    };
    match catalog_result {
        Ok(mut catalog) => {
            let command = Command::parse();
            if let Some(command) = command.commands {
                match command {
                    Commands::List => list(&catalog),
                    Commands::Import { file } => match Catalog::from_file(&file) {
                        Ok(catalog) => {
                            list(&catalog);
                            save_catalog(&catalog, &database);
                        }
                        Err(err) => eprintln!("error: {}", err),
                    },
                    Commands::Add {
                        sub_category,
                        category,
                    } => match catalog.add_sub_category(&sub_category, &category) {
                        Ok(_) => {
                            println!("added {} to {}", sub_category, category);
                            list(&catalog);
                            save_catalog(&catalog, &database);
                        }
                        Err(err) => eprintln!("error: {}", err),
                    },
                    Commands::Move {
                        sub_category,
                        category,
                    } => match catalog.move_sub_category(&sub_category, &category) {
                        Ok(_) => {
                            println!("moved {} to {}", sub_category, category);
                            list(&catalog);
                            save_catalog(&catalog, &database);
                        }
                        Err(err) => eprintln!("error: {}", err),
                    },
                    Commands::Remove { category, force } => {
                        match catalog.remove_category(&category, force) {
                            Ok(_) => {
                                println!("removed {}", category);
                                list(&catalog);
                                save_catalog(&catalog, &database);
                            }
                            Err(err) => eprintln!("error: {}", err),
                        }
                    }
                }
            } else {
                list(&catalog)
            }
        }
        Err(e) => {
            println!("can't open catalog: {}", e);
        }
    }
}
