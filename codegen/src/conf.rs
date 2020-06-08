use serde::Deserialize;
use toml::map::Map;
use toml::Value;
use std::fs::File;
use std::io::Read;
use crate::CONF_FILE_PATH;

#[derive(Deserialize)]
#[derive(Debug)]
pub(crate) struct Basic {
    pub(crate) derive: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub(crate) struct StructEntry {
    pub(crate) name: String,
    pub(crate) tagged: String,
    pub(crate) fields: Option<Map<String, Value>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
pub(crate) struct Conf{
    pub(crate) basic: Basic,
    pub(crate) structs: Vec<StructEntry>,
}

impl Conf {

    pub fn read_config() -> Self {
        let file_path = CONF_FILE_PATH;
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e)
        };

        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e)
        };
        // Conditions for Unwrap:
        //     Struct field names must correspond to toml configuration files
        toml::from_str::<Conf>(&str_val).unwrap()
    }

}