use std::fs::{File, read_dir, remove_file};
use std::io::{Write, Result, Read};
use serde_json::{to_string,from_str};
use std::fs;

use crate::rainbow_table::Node;

use crate::constants::{SIZE, NB_PASSWORD, NB_NODE};


// Cette fonction prend en argument un vecteur (dans notre cas une rainbow table) et écrit dans un fichier .json le contenu 
// de cette table en le nomant avec les constantes définies.
pub fn serialize<T>(data: &Vec<T>) -> Result<()>
where
    T: serde::Serialize,
{
    println!("Save file...");

    let json_string = to_string(data)?;

    let mut file = File::create(&format!("./data/RainbowTable_{}_{}_{}.json", SIZE, *NB_PASSWORD, *NB_NODE))?;

    file.write_all(json_string.as_bytes())?;

    file.flush()?;

    Ok(())
}

// Cette fonction va chercher dans le dossier data le fichier .json correspondant aux constantes actuelles et récupère
// son contenu puis le transforme en vecteur de node qui sera une rainbow table.
pub fn deserialize() -> Result<Vec<Node>> {

    let mut file = File::open(format!("./data/RainbowTable_{}_{}_{}.json", SIZE, *NB_PASSWORD, *NB_NODE))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let nodes: Vec<Node> = from_str(&contents)?;

    Ok(nodes)
}


// Cette fonction renvoie true si un fichier portant le nom filename se trouve dans le dossier directory et false sinon.
pub fn file_exists_in_directory(directory: &str, filename: &str) -> bool {
    if let Ok(files) = read_dir(directory) {
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

// Cette fonction supprime le fichier filename dans le dossier directory.
pub fn delete_file_in_directory(directory: &str, filename: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(directory).join(filename);
    remove_file(path)?;
    Ok(())
}

// Supprime tout les fichiers contenu dans le dossier directory
pub fn delete_all_file_in_directory(directory: &str) {
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Err(e) = fs::remove_file(path) {
                        println!("Error deleting file: {:?}", e);
                    }
                }
            } else if let Err(e) = entry {
                println!("Error reading directory entry: {:?}", e);
            }
        }
    } else {
        println!("Error reading directory: {}", directory);
    }
}