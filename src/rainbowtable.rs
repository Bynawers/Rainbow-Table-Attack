use crate::sha3::sha3;
use crate::reduction::reduce_truncate_xor;

const NONCE: u32 = 248820715;

const NB_PASSWORD: u32 = 36;
const NB_NODE: u32 = 4;


#[derive(Debug)]
#[derive(Clone)]
struct Node {
    start: String,
    end: String,
}


fn generate_table(rainbow_table: &mut Vec<Node>, start: &str, nb_node: u32, nb_password: u32) {
    let mut hash = sha3(start);
    let mut reduce;
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for i in 0..nb_password {
        for j in 0..nb_node {
            reduce = reduce_truncate_xor(hash.as_slice().try_into().unwrap(), j+NONCE);
            hash = sha3(&reduce.clone());
            
            if j == 0 {
                node.start = reduce.clone();
            } else if j+1 == nb_node {
                node.end = reduce.clone();
            }
        }
        rainbow_table.push(node.clone());
    }
    println!(" > {} ROUND :\n", NB_NODE);

    print_table(rainbow_table)
}

fn print_table(rainbow_table: &Vec<Node>) {

    for element in rainbow_table {
        println!("{:?} \n", element);
    }
}

pub fn rainbowtable(password : &str) {

    let mut rainbow_table: Vec<Node> = Vec::new();

    generate_table(&mut rainbow_table, password, NB_NODE, NB_PASSWORD);

    //print_table(&rainbow_table);
}
