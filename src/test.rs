use crate::rainbow_table;
use crate::sha3::sha3;
use crate::reduction::reduction;

use crate::constants::*;

const NB_PASSWORD_TOTAL: u64 = (SIGMA_SIZE as u64).pow(SIZE as u32);

pub fn percent_of_password(rainbow_table: &Vec<rainbow_table::Node>) {

    let mut all_passw = Vec::<String>::new();

    for elt in rainbow_table {
        let mut red = elt.start.clone();

        if !contains(&red,&all_passw) {
            all_passw.push(String::from(red.clone()));
        }

        for i in 1..NB_NODE {
            let hash = sha3(&red.clone());

            red = reduction(hash, i+NONCE);
            
            if !contains(&red, &all_passw) {
                all_passw.push(String::from(red.clone()));
            }
        }
    }
    println!("pourcentage des mdp testés : {}%",(all_passw.len() as f32 / NB_PASSWORD_TOTAL as f32 *100.0));
}

/*
fn _test2() {
    let mut red = String::from("7");

    print!("test juste avec {}\n",red);
    println!("start : {}",red);

    for i in 1..NB_NODE {
        let hash = sha3(&red);
        //affiche_hash(hash);
        red = reduction(hash, i+NONCE);

        if i != NB_NODE -1 {
            println!("étape intermédiaire de la ligne : {}",red);
        }
    }
    println!("end {}",red);
}*/

fn contains(truc:&str, vector:&Vec<String>) -> bool {
    for elt in vector {
        if truc == elt {
            return true;
        }
    }
    return false;
}