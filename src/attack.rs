use crate::sha3;
use crate::reduction;
use crate::rainbow_table;
use crate::constants;
use crate::test;

pub fn execution() {
    
    let mut rainbow_table: Vec<rainbow_table::Node> = Vec::new();

    rainbow_table::generate_table(&mut rainbow_table, constants::NB_NODE, constants::NB_PASSWORD);

    test::test(&rainbow_table);

    //reduce : ab -> w2 -> jh -> do
    //print_table(&rainbow_table);
    println!("\nAttack réussi ? {}", search_password(&mut rainbow_table, constants::FLAG));
}


fn search_password(rainbow_table: &mut Vec<rainbow_table::Node>, flag: &str) -> bool {

    println!("\nsearch {}", flag);

    let hash_flag = sha3::digest(flag);
    let mut position_flag = 0;

    let mut reduce = reduction::reduce_truncate_xor(hash_flag, constants::NONCE);

    //println!("\n> nb_node = {} :", constants::NB_NODE);
    //println!("> nb_password = {} :", constants::NB_PASSWORD);
    //println!("> hash_flag = {} :", reduce);

    for i in 0..constants::NB_NODE {
        //println!("\n> essai {} :", i);

        // RN
        // RN-1 RN
        // RN-2 RN-1 RN...
        for j in constants::NB_NODE-(i+1)..constants::NB_NODE {
            
            let mut tmp = sha3::digest(&reduce.clone());
            reduce = reduction::reduce_truncate_xor(tmp, j+constants::NONCE);

            if j+1 == constants::NB_NODE {
                //print!("{} (R{})", reduce, j);
            }
        }

        //println!("search {}", reduce);
        position_flag = compare_end(rainbow_table, reduce.clone());
        if position_flag > 0 {
            reverse(rainbow_table, hash_flag, position_flag);
            return true;
        }
    }
    return false;
}

/*
*   Compare la value d'entrée à toute les valeurs "end" de la rainbowtable passé en entrée
*/
fn compare_end(rainbow_table: &mut Vec<rainbow_table::Node>, value: String) -> u32 {

    for i in 0..constants::NB_PASSWORD {
        if rainbow_table[i as usize].end == value {
            println!("find {} ! position {}", value, i);
            return i;
        }
    }
    return 0;
}

fn reverse(rainbow_table: &mut Vec<rainbow_table::Node>, hash_flag: [u8; 32], position_flag: u32) {

    println!("Index {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    let mut reduce = rainbow_table[position_flag as usize].start.clone();
    let mut hash = sha3::digest(&reduce);

    for i in 1..constants::NB_NODE+1 {

        print!("{} => ", reduce);

        if hash == hash_flag {
            println!("THE FLAG IS : {}", reduce);
        }

        reduce = reduction::reduce_truncate_xor(hash, i+constants::NONCE);
        hash = sha3::digest(&reduce);
    }

}