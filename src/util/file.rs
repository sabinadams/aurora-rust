use crate::vars::CONFIG_PATH;
use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::read_to_string;
use std::io;
use crate::model_helpers;
#[derive(Deserialize, Serialize, Debug)]
pub struct AuroraConfig {
    pub files: Vec<String>,
    pub output: String,
}

pub enum MyError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => MyError::Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => MyError::Json(err),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        Self::Io(err)
    }
}

/// Reads in the aurora.config.json file and parses it to JSON
pub fn read_aurora_config() -> Result<AuroraConfig, MyError> {
    let raw = read_to_string(CONFIG_PATH)?;
    let test = from_str::<AuroraConfig>(&raw)?;
    Ok(test)
}

/// Takes a set of paths/blobs and reads all of the prisma files they point to or that match the blob pattern.
/// <br/> This function also ensures each schema is valid, otherwise it throws an error.
pub fn read_all_schemas(paths: Vec<String>) -> Vec<(std::path::PathBuf, std::string::String)> {
    let mut schemas: Vec<(std::path::PathBuf, std::string::String)> = vec![];

    for path in paths {
        for entry in glob(&path).unwrap_or_else(|_err| {
            eprintln!("Invalid glob pattern: {}", path);
            std::process::exit(0)
        }) {
            if let Ok(path) = entry {
                let schema = read_to_string(path.clone()).unwrap_or_else(|_err| {
                    eprintln!("Could not read the schema at {}", path.display());
                    std::process::exit(0)
                });
                match model_helpers::validate_schema(schema.clone()) {
                    Ok(_) => schemas.push((
                        path,
                        schema
                    )),
                    Err(errs) => {
                        eprintln!("[AURORA]: {}", errs.to_pretty_string(&path.display().to_string(), &schema));
                        std::process::exit(0)
                    }
                }
            }
        }
    }

    schemas
}
