use crate::rainbow_table;
use crate::sha3;
use crate::reduction;

use crate::constants::*;

const NB_PASSWORD_TOTAL: u64 = (SIGMA_SIZE as u64).pow(SIZE as u32);

pub fn test(rainbow_table: &Vec<rainbow_table::Node>) {

    let mut all_passw = Vec::<String>::new();

    for elt in rainbow_table {
        let mut red = elt.start.clone();

        if !contains(&red,&all_passw) {
            all_passw.push(String::from(red.clone()));
        }

        for i in 1..NB_NODE {
            let hash = sha3::digest(&red.clone());

            red = reduction::reduce_xor(hash, i+NONCE);
            
            if !contains(&red,&all_passw) {
                all_passw.push(String::from(red.clone()));
            }
            //print!("valeur de i : {}    ",i);
            if i+1 != NB_NODE {
                //println!("étape intermédiaire de la ligne : {}",red);
            }
        }
        //println!("  end {}",red);
    }
    //println!("{:?}",all_passw);
    //test2();
    println!("pourcentage des mdp testés : {}%",(all_passw.len() as f32 / NB_PASSWORD_TOTAL as f32 *100.0));
}

fn test2() {
    let mut red = String::from("7");

    print!("test juste avec {}\n",red);
    println!("start : {}",red);

    for i in 1..NB_NODE {
        let hash = sha3::digest(&red);
        //affiche_hash(hash);
        red = reduction::reduce_xor(hash, i+NONCE);

        if i != NB_NODE -1 {
            println!("étape intermédiaire de la ligne : {}",red);
        }
    }
    println!("end {}",red);
}

fn contains(truc:&str,vector:&Vec<String>) -> bool {
    for elt in vector {
        if truc == elt {
            return true;
        }
    }
    return false;
}