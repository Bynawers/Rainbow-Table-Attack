use crate::attack;
use crate::constants;
use crate::sha3;
use crate::reduction;
use crate::rainbow_table;



/*
pub fn test(rainbow_table: &Vec<attack::Node>) {
    let mut deja_fait = Vec::<&str>::new();
    for elt in rainbow_table {
        if !contains(&elt.start,&deja_fait) {
            deja_fait.push(&elt.start);
        }
        if !contains(&elt.end,&deja_fait) {
            deja_fait.push(&elt.end);
        }
    }
    println!("pourcentage des mdp testés : {}",(deja_fait.len()as f32 / 36.0)*100.0);
}

fn contains(truc:&str, vector:&Vec<&str>) -> bool {
    for elt in vector {
        if &truc == elt {
            return true;
        }
    }
    false
}*/

/*
pub fn generate_node(rainbow_table: &Vec<attack::Node>) -> bool {

    for i in 0..constants::NB_PASSWORD {
        println!("\n> node {} :", i);
        

        println!("\nsearch {}", reduce);
    }
}*/

pub fn test(rainbow_table: &Vec<rainbow_table::Node>) {

    let mut all_passw = Vec::<String>::new();

    for elt in rainbow_table {
        let mut red = elt.start.clone();
        all_passw.push(String::from(&red));

        for i in 1..constants::NB_NODE {
            let hash = sha3::digest(&red);
            red = reduction::reduce_truncate_xor(hash, i+constants::NONCE);
            all_passw.push(String::from(&red));
        }

        break;
    }
    //println!("test : {:?}", all_passw);
    //println!("rainbow : {:?}", rainbow_table[0]);
}