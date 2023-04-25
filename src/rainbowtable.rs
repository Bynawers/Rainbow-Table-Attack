use crate::constants::SIZE;
use crate::sha3;
use random_string::generate;

use crate::reduction;
use crate::constants;

pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub start: String,
    pub end: String,
}

pub fn generate_table(message: &str) -> Vec<Node> {
    println!("rainbow tables pdt leur création :");
    let mut rainbow_table : Vec<Node> = vec![];
    let mut hash = sha3::sha3(message);
    let mut reduce = generate(SIZE as usize,CHARSET);
    let mut starting_items = Vec::<String>::new();
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..constants::NB_PASSWORD {
        for j in 0..constants::NB_NODE {
            if j == 0 { 
                reduce = reduction::reduce_xor(hash,j+constants::NONCE);
                while contains(reduce.to_string(),&mut starting_items) {
                    reduce = generate(SIZE as usize,CHARSET);
                }
                println!("  start : {}",reduce);
                node.start = reduce.to_string();
                starting_items.push(reduce.to_string());
            } else if j+1 == constants::NB_NODE {
                hash = sha3::sha3(&reduce);
                reduce = reduction::reduce_xor(hash,j+constants::NONCE);
                println!("  end : {}",reduce);
                node.end = String::from(reduce.to_string());
            } else {
                hash = sha3::sha3(&reduce);
                reduce = reduction::reduce_xor(hash,j+constants::NONCE);
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

