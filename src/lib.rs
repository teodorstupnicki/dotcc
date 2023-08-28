use std::{error::Error, fs};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration<'a> {
    pub url: &'a str
}

impl<'a> Configuration<'a> {
    pub fn new(path: &'a str) -> Result<Configuration, Box<dyn Error>> {
        let deserialized = serde_json::from_str::<Configuration>(path)?;
        Ok(deserialized)
    }
}

pub fn read_config<'a>(path: &'a str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}