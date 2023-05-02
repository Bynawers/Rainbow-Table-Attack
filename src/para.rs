use rayon::prelude::*;
use std::sync::mpsc::channel;
use num_cpus;

use serde::{Serialize, Deserialize};
use random_string::generate;



use crate::sha3::sha3;
use crate::reduction::reduction;
use crate::rainbow_table::{Node};

use crate::constants::*;


fn test() {
    let num_threads = num_cpus::get();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    let v: Vec<i32> = (0..1000).collect();
    let result: Vec<i32> = pool.install(|| v.par_iter().map(|x| x * 2).collect());

    println!("{:?}", result);
}

pub fn pool() -> Vec<Node> {
    let num_threads = num_cpus::get();
    println!("{}", num_threads);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = NB_PASSWORD / num_threads as u32;
    let table: Vec<Node> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { NB_PASSWORD } else { start + slice };
                generate_table(start,end)
            })
            .flatten().collect()
    });
    table
}
//pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";
pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn generate_table(startpassword : u32,endpassword : u32) -> Vec<Node> {
    println!("{} start ,{} end ", startpassword ,endpassword);
    let mut rainbow_table : Vec<Node> = vec![];
    let mut hash = sha3(GENERATOR_RAINBOW_TABLE);
    let mut reduce = generate(SIZE as usize,CHARSET);
    let mut starting_items = Vec::<String>::new();

    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for _ in startpassword..endpassword {
        for j in 0..NB_NODE {
            if j == 0 { 
                reduce = reduction(hash,j+NONCE);

                while contains(reduce.to_string(),&mut starting_items) {
                    reduce = generate(SIZE as usize,CHARSET);
                }
                node.start = reduce.to_string();
                starting_items.push(reduce.to_string());

            } 
            else if j+1 == NB_NODE {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE);
                node.end = String::from(reduce.to_string());
            } 
            else {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE);
                //print!("valeur de j : {}    ",j);
            }
        }
        rainbow_table.push(node.clone());
    }
    println!("fin");
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