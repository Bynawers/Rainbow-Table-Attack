use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Result, Read};
use random_string::generate;
use serde_json;

use crate::sha3::sha3;
use crate::reduction;

use crate::constants::*;

pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub start: String,
    pub end: String,
}

pub fn generate_table() -> Vec<Node> {


    let mut rainbow_table : Vec<Node> = vec![];
    let mut hash = sha3(GENERATOR_RAINBOW_TABLE);
    let mut reduce = generate(SIZE as usize,CHARSET);
    let mut starting_items = Vec::<String>::new();

    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..NB_PASSWORD {
        for j in 0..NB_NODE {
            if j == 0 { 
                reduce = reduction::reduce_xor(hash,j+NONCE);
                while contains(reduce.to_string(),&mut starting_items) {
                    reduce = generate(SIZE as usize,CHARSET);
                }
                println!("  start : {}",reduce);
                node.start = reduce.to_string();
                starting_items.push(reduce.to_string());

            } 
            else if j+1 == NB_NODE {
                hash = sha3(&reduce);
                reduce = reduction::reduce_xor(hash,j+NONCE);
                println!("  end : {}",reduce);
                node.end = String::from(reduce.to_string());
            } 
            else {
                hash = sha3(&reduce);
                reduce = reduction::reduce_xor(hash,j+NONCE);
                //print!("valeur de j : {}    ",j);
                println!("étape intermédiaire : {}",reduce);
            }
        }
        rainbow_table.push(node.clone());
    }
    rainbow_table
    //println!("{:?}",starting_items);
}

fn contains(elt:String, tab: &mut Vec::<String>) -> bool {
    for mdp in tab {
        if mdp == &elt {
            return true;
        }
    }
    return false;
}

pub fn serialize<T>(data: &Vec<T>) -> Result<()>
where
    T: serde::Serialize,
{
    let json_string = serde_json::to_string(data)?;

    let mut file = File::create(RAINBOW_TABLE_PATH)?;

    file.write_all(json_string.as_bytes())?;

    file.flush()?;

    Ok(())
}

pub fn deserialize() -> Result<Vec<Node>> {

    let mut file = File::open(RAINBOW_TABLE_PATH)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let nodes: Vec<Node> = serde_json::from_str(&contents)?;

    Ok(nodes)
}