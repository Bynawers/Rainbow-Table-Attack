use crate::constants::SIZE;
use crate::sha3;
use random_string::generate;

use crate::reduction;
use crate::constants;
use crate::attack;

pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub start: String,
    pub end: String,
}

pub fn generate_table(rainbow_table: &mut Vec<Node>, message: &str, nb_node: u32, nb_password: u32) {
    println!("rainbow tables pdt leur création :");
    let mut hash = sha3::sha3(message);
    let mut reduce = Box::leak(generate(SIZE as usize,CHARSET).into_boxed_str());
    let mut starting_items = Vec::<String>::new();
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password {
        for j in 0..nb_node {
            if j == 0 { 
                reduce = Box::leak(reduction::reduce_truncate_xor(hash,2*j+constants::NONCE).into_boxed_str());
                while contains(reduce.to_string(),&mut starting_items) {
                    reduce = Box::leak(generate(SIZE as usize,CHARSET).into_boxed_str());
                }
                //println!("  start : {}",reduce);
                node.start = reduce.to_string();
                starting_items.push(reduce.to_string());
            } else if j+1 == nb_node {
                hash = sha3::sha3(reduce);
                reduce = Box::leak(reduction::reduce_truncate_xor(hash,2*j+constants::NONCE).into_boxed_str());
                //println!("  end : {}",reduce);
                node.end = String::from(reduce.to_string());
            } else {
                hash = sha3::sha3(reduce);
                reduce = Box::leak(reduction::reduce_truncate_xor(hash,2*j+constants::NONCE).into_boxed_str());
                //print!("valeur de j : {}    ",j);
                //println!("étape intermédiaire : {}",reduce);
            }
        }
        rainbow_table.push(node.clone());
    }
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