use serde::{Serialize, Deserialize};

use std::io::{self, prelude::*};
use std::error::Error;
use std::fmt;
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Database {
    values: Vec<i32>,
}

// OpenDatabaseError {{{

#[derive(Debug)]
enum OpenDatabaseError {
    Io(io::Error),
    Deserialize(serde_json::Error),
}

impl fmt::Display for OpenDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenDatabaseError::Io(_) => write!(f, "io error"),
            OpenDatabaseError::Deserialize(_) => write!(f, "deserialization error"),
        }
    }
}

impl Error for OpenDatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            OpenDatabaseError::Io(err) => Some(err),
            OpenDatabaseError::Deserialize(err) => Some(err),
        }
    }
}

impl From<io::Error> for OpenDatabaseError {
    fn from(err: io::Error) -> OpenDatabaseError {
        OpenDatabaseError::Io(err)
    }
}

impl From<serde_json::Error> for OpenDatabaseError {
    fn from(err: serde_json::Error) -> OpenDatabaseError {
        OpenDatabaseError::Deserialize(err)
    }
}

// }}}

const DB_PATH: &str = "./database.json";

fn open_database() -> Result<Database, OpenDatabaseError> {
    let mut file = File::open(DB_PATH)?;
    /*
    let mut file = match File::open(DB_PATH) {
        Ok(file) => file,
        Err(err) => return Err(err.into()),
    };
    */
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let db = serde_json::from_str(&file_content)?;

    // return Ok(db);
    Ok(db)
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        if let Some(source_err) = err.source() {
            println!("caused by: {}", source_err);
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let db = open_database()?;
    println!("database: {:#?}", db);

    Ok(())
}

// vim: fdm=marker
