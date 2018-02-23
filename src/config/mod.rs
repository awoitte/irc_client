//#[macro_use]
extern crate json;

use std::io::*;
use std::str::FromStr;

pub struct Config{
    pub name: String
}

pub fn get_config_from_string (json_string: &str) -> Result<Config>{
    let parsed = json::parse(json_string).unwrap();
    let name_property = &parsed["name"];
    Ok(Config{
        name: String::from(name_property.as_str().unwrap())
    })
}

pub fn default_config() -> Config{
    Config {
        name: String::from("guest")
    }
}

