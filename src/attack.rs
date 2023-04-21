use sha3::{Sha3_256, Digest};

use crate::reduction;
use crate::rainbow_table;
use crate::constants;

pub fn execution() {
    let message = "Cryptographie";
    let flag = "psg";

    let mut rainbow_table: Vec<rainbow_table::Node> = Vec::new();

    rainbow_table::generate_table(&mut rainbow_table, message, constants::NB_NODE, constants::NB_PASSWORD);
    //print_table(&rainbow_table);
    println!("\nAttack réussi ? {}", search_password(&mut rainbow_table, flag, constants::NB_NODE, constants::NB_PASSWORD));
}




fn search_password(rainbow_table: &mut Vec<rainbow_table::Node>, flag: &str, nb_node: u32, nb_password: u32) -> bool {

    let hash_flag = Sha3_256::digest(flag.as_bytes());
    let mut reduce = reduction::reduce_xor(hash_flag.as_slice().try_into().unwrap(), constants::NONCE);

    println!("\n> nb_node = {} :", nb_node);
    println!("> nb_password = {} :", nb_password);
    println!("> hash_flag = {} :", reduce);

    for i in 0..nb_node {
        println!("\n> node {} :", i);

        for j in nb_node-(i+1)..nb_node {
            
            let mut tmp = Sha3_256::digest(reduce.clone());
            reduce = reduction::reduce_xor(tmp.as_slice().try_into().unwrap(), j+constants::NONCE);

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

fn compare_end(rainbow_table: &mut Vec<rainbow_table::Node>, value: String, nb_password: u32) -> bool {

    for i in 0..nb_password {
        if rainbow_table[i as usize].end == value {
            return true;
        }
    }

    return false;
}

fn print_table(rainbow_table: &Vec<rainbow_table::Node>) {

    for element in rainbow_table {
        println!("{:?} \n", element);
    }
}