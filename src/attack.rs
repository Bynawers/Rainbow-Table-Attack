use sha3::{Sha3_256, Digest};

use crate::reduction;

const NONCE: u32 = 248820715;

const NB_PASSWORD: u32 = 10;
const NB_NODE: u32 = 5;

#[derive(Debug)]
#[derive(Clone)]
struct Node {
    start: String,
    end: String,
}

pub fn execution() {
    let message = "Cryptographie";
    let flag = "psg";

    let mut rainbow_table: Vec<Node> = Vec::new();

    generate_table(&mut rainbow_table, message, NB_NODE, NB_PASSWORD);
    //print_table(&rainbow_table);
    println!("\nAttack réussi ? {}", search_password(&mut rainbow_table, flag, NB_NODE, NB_PASSWORD));
}

fn generate_table(rainbow_table: &mut Vec<Node>, message: &str, nb_node: u32, nb_password: u32) {
    let mut hash = Sha3_256::digest(message.as_bytes());
    let mut reduce;
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password {
        for j in 0..nb_node {
            reduce = reduction::reduce_truncate(hash.as_slice().try_into().unwrap(), i+NONCE);
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

fn search_password(rainbow_table: &mut Vec<Node>, flag: &str, nb_node: u32, nb_password: u32) -> bool {

    let hash_flag = Sha3_256::digest(flag.as_bytes());
    let mut reduce = reduction::reduce_xor(hash_flag.as_slice().try_into().unwrap(), NONCE);

    println!("\n> nb_node = {} :", nb_node);
    println!("> nb_password = {} :", nb_password);
    println!("> hash_flag = {} :", reduce);

    for i in 0..nb_node {
        println!("\n> node {} :", i);

        for j in nb_node-(i+1)..nb_node {
            
            let mut tmp = Sha3_256::digest(reduce.clone());
            reduce = reduction::reduce_xor(tmp.as_slice().try_into().unwrap(), j+NONCE);

            if j+1 == nb_node {
                print!("{} (R{})", reduce, j);
            }
            else {
                print!("{} (R{}) => ", reduce, j);
            }
        }

        println!("\nsearch {}", reduce);
        if compare_end(rainbow_table, reduce.clone(), nb_password) {
            return true;
        }
    }
    return false;
}

fn compare_end(rainbow_table: &mut Vec<Node>, value: String, nb_password: u32) -> bool {

    for i in 0..nb_password {
        if rainbow_table[i as usize].end == value {
            return true;
        }
    }

    return false;
}

fn print_table(rainbow_table: &Vec<Node>) {

    for element in rainbow_table {
        println!("{:?} \n", element);
    }
}