use serde::{Serialize, Deserialize};
use random_string::generate;

use crate::sha3::sha3;
use crate::reduction::reduction;

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

    for _ in 0..NB_PASSWORD {
        for j in 0..NB_NODE {
            if j == 0 { 
                reduce = reduction(hash,j+NONCE);

                while contains(reduce.to_string(),&mut starting_items) {
                    reduce = generate(SIZE as usize,CHARSET);
                }
                node.start = reduce.to_string();
                starting_items.push(reduce.to_string());

            } 
            else if j+1 == NB_NODE {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE);
                node.end = String::from(reduce.to_string());
            } 
            else {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE);
                //print!("valeur de j : {}    ",j);
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