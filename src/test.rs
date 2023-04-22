use crate::rainbow_table;
use crate::constants;
use crate::sha3;
use crate::reduction;

pub fn test(rainbow_table: &Vec<rainbow_table::Node>) {
    let mut all_passw = Vec::<String>::new();
    for elt in rainbow_table {
        let mut red = elt.start.clone();
        all_passw.push(String::from(&red));
        println!("start {}",red);
        for i in 0..constants::NB_NODE {
            let hash = sha3::sha3(&red);
            red = reduction::reduce_xor(hash, i+constants::NONCE);
            println!("{}",red);
            all_passw.push(String::from(&red));
            }
        println!("end {}",red);
    }
    println!("{:?}",all_passw);
    //println!("pourcentage des mdp testés : {}",(all_passw.len() as f32 / constants::NB_PASSWORD as f32)*100.0);
}

fn contains(truc:&str,vector:&Vec<&str>) -> bool {
    for elt in vector {
        if &truc == elt {
            return true;
        }
    }
    false
}