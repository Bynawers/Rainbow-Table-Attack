use crate::rainbow_table;
use crate::constants;
use crate::sha3;
use crate::reduction;
use crate::attack;

pub fn test(rainbow_table: &Vec<rainbow_table::Node>) {
    println!("try de recréer la rainbow table a partir du start de chaque ligne :");
    let mut all_passw = Vec::<String>::new();
    for elt in rainbow_table {
        let mut red = elt.start.clone();
        all_passw.push(String::from(&red));
        println!("  start {}",red);
        for i in 1..constants::NB_NODE {
            let hash = sha3::sha3(&red);
            //if i == 0 {
            //    attack::affiche_hash(hash);
            //}
            red = reduction::reduce_xor(hash, i+constants::NONCE);
            //print!("valeur de i : {}    ",i);
            if i != constants::NB_NODE -1 {
                println!("étape intermédiaire de la ligne : {}",red);
            }
            all_passw.push(String::from(&red));
            }
        println!("  end {}",red);
    }
    //println!("{:?}",all_passw);
    test2();
    //println!("pourcentage des mdp testés : {}",(all_passw.len() as f32 / constants::NB_PASSWORD as f32)*100.0);
}

fn test2() {
    let mut red = String::from("c");
    print!("test juste avec c \n");
    println!("start : {}",red);
    for i in 1..constants::NB_NODE {
        let hash = sha3::sha3(&red);
        red = reduction::reduce_xor(hash, i+constants::NONCE);
        if i != constants::NB_NODE -1 {
            println!("étape intermédiaire de la ligne : {}",red);
        }
    }
    println!("end {}",red);
}

fn contains(truc:&str,vector:&Vec<&str>) -> bool {
    for elt in vector {
        if &truc == elt {
            return true;
        }
    }
    false
}