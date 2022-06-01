use crate::vars::CONFIG_PATH;
use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fs::read_to_string;
use std::io;

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

pub fn read_aurora_config() -> Result<AuroraConfig, MyError> {
    let raw = read_to_string(CONFIG_PATH)?;
    let test = from_str::<AuroraConfig>(&raw)?;
    Ok(test)
}

pub fn read_all_schemas(paths: Vec<String>) -> Vec<String> {
    let mut schemas: Vec<String> = vec![];

    for path in paths {
        for entry in glob(&path).unwrap_or_else(|_err| {
            eprintln!("Invalid glob pattern: {}", path);
            std::process::exit(0)
        }) {
            if let Ok(path) = entry {
                schemas.push(read_to_string(path.clone()).unwrap_or_else(|_err| {
                    eprintln!("Could not read the schema at {}", path.display());
                    std::process::exit(0)
                }))
            }
        }
    }

    schemas
}