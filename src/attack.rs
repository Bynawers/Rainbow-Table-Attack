use crate::sha3;

use crate::test;

use crate::reduction;

const NONCE: u32 = 248820715;

const NB_PASSWORD: u32 = 36;
const NB_NODE: u32 = 4;

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    pub start: String,
    pub end: String,
}

pub fn execution() {
    let message = "a";
    

    let mut rainbow_table: Vec<Node> = Vec::new();

    generate_table(&mut rainbow_table, message, NB_NODE, NB_PASSWORD);

    print_table(&rainbow_table);
    test::test(&rainbow_table);
}

fn generate_table(rainbow_table: &mut Vec<Node>, start: &str, nb_node: u32, nb_password: u32) {
    let mut hash = sha3::sha3(start);
    let mut reduce;
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password {
        println!("tour : {}",i);
        for j in 0..nb_node {
            reduce = reduction::reduce_xor(hash.as_slice().try_into().unwrap(), i+NONCE);
            hash = sha3::sha3(&reduce.clone());
            
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