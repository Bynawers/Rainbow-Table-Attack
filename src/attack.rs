use sha3::{Sha3_256, Digest};

use crate::reduction;

const NONCE: u32 = 248820715;

const NB_PASSWORD: u32 = 100;
const NB_NODE: u32 = 100;

#[derive(Debug)]
#[derive(Clone)]
struct Node {
    start: String,
    end: String,
}

pub fn execution() {
    let message = "Cryptographie";
    

    let mut rainbow_table: Vec<Node> = Vec::new();

    generate_table(&mut rainbow_table, message, NB_NODE, NB_PASSWORD);

    print_table(&rainbow_table);
}

fn generate_table(rainbow_table: &mut Vec<Node>, start: &str, nb_node: u32, nb_password: u32) {
    let mut hash = Sha3_256::digest(start.as_bytes());
    let mut reduce;
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password {
        for j in 0..nb_node {
            reduce = reduction::reduce_xor(hash.as_slice().try_into().unwrap(), i+NONCE);
            hash = Sha3_256::digest(reduce.clone());
            
            if j == 0 {
                node.start = reduce.clone();
            } else if j+1 == nb_node {
                node.end = reduce.clone();
            }
        }
        rainbow_table.push(node.clone());
    }
    println!(" > {} ROUND :\n", NB_NODE);
}

fn print_table(rainbow_table: &Vec<Node>) {

    for element in rainbow_table {
        println!("{:?} \n", element);
    }
}