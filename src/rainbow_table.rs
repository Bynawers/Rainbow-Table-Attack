use crate::sha3;

use crate::reduction;
use crate::constants;

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub start: String,
    pub end: String,
}

pub fn generate_table(rainbow_table: &mut Vec<Node>, message: &str, nb_node: u32, nb_password: u32) {
    println!("rainbow tables pdt leur création :");
    let mut hash = sha3::sha3(message);
    let mut reduce;
    let mut starting_items = Vec::<String>::new();
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password/nb_node {
        for j in 0..nb_node {
            reduce = reduction::reduce_xor(hash, j+constants::NONCE);
            hash = sha3::sha3(&reduce);
            //print!("valeur de j : {}    ",j);
            if (j!=0) && (j+1!=nb_node) {
                println!("étape intermédiaire : {}",reduce);
            }

            if j == 0 { 
                while contains(reduce.clone(),&mut starting_items) {
                    hash = sha3::sha3(&reduce);
                    reduce = reduction::reduce_xor(hash, j+constants::NONCE);
                }
                println!("  start : {}",reduce);
                node.start = reduce.clone();
                starting_items.push(reduce);
            } else if j+1 == nb_node {
                println!("  end : {}",reduce);
                node.end = reduce.clone();
            }
        }
        rainbow_table.push(node.clone());
    }
}

fn contains(elt:String, tab: &mut Vec::<String>) -> bool {
    for mdp in tab {
        if mdp == &elt {
            return true;
        }
    }
    return false;
}