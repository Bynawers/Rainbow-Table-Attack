use crate::sha3;
use crate::reduction;
use crate::constants;

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub start: String,
    pub end: String,
}

pub fn generate_table(rainbow_table: &mut Vec<Node>, nb_node: u32, nb_password: u32) {
    let mut hash = sha3::digest(constants::GENERATOR_RAINBOW_TABLE);
    let mut reduce;
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password {
        for j in 0..nb_node {
            reduce = reduction::reduce_truncate_xor(hash.as_slice().try_into().unwrap(), j+constants::NONCE);
            hash = sha3::digest(&reduce.clone());
            
            //if i == 1 { print!("{} => ", reduce); }
            if j == 0 {
                node.start = reduce.clone();
            } else if j+1 == nb_node {
                node.end = reduce.clone();
            }
        }
        rainbow_table.push(node.clone());
    }
}