use sha3::{Sha3_256, Digest};

use crate::reduction;
use crate::constants;

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub start: String,
    pub end: String,
}

pub fn generate_table(rainbow_table: &mut Vec<Node>, message: &str, nb_node: u32, nb_password: u32) {
    let mut hash = Sha3_256::digest(message.as_bytes());
    let mut reduce;
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password {
        for j in 0..nb_node {
            reduce = reduction::reduce_truncate(hash.as_slice().try_into().unwrap(), i+constants::NONCE);
            hash = Sha3_256::digest(reduce.clone());
            
            if j == 0 {
                node.start = reduce.clone();
            } else if j+1 == nb_node {
                node.end = reduce.clone();
            }
        }
        rainbow_table.push(node.clone());
    }
}