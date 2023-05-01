use std::fs::File;
use std::fs;
use std::io::{Write, Result, Read};
use serde_json;

use crate::rainbow_table::Node;

use crate::constants::*;

pub fn serialize<T>(data: &Vec<T>) -> Result<()>
where
    T: serde::Serialize,
{
    println!("Save file...");

    let json_string = serde_json::to_string(data)?;

    let mut file = File::create(&format!("./data/RainbowTable_{}_{}_{}.json", SIZE, NB_PASSWORD, NB_NODE))?;

    file.write_all(json_string.as_bytes())?;

    file.flush()?;

    Ok(())
}

pub fn deserialize() -> Result<Vec<Node>> {

    let mut file = File::open(format!("./data/RainbowTable_{}_{}_{}.json", SIZE, NB_PASSWORD, NB_NODE))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let nodes: Vec<Node> = serde_json::from_str(&contents)?;

    Ok(nodes)
}

pub fn file_exists_in_directory(directory: &str, filename: &str) -> bool {
    if let Ok(files) = fs::read_dir(directory) {
        for file in files {
            if let Ok(file) = file {
                if let Some(name) = file.file_name().to_str() {
                    if name == filename {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn delete_file_in_directory(directory: &str, filename: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(directory).join(filename);
    fs::remove_file(path)?;
    Ok(())
}