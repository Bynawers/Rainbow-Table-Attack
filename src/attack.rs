use crate::sha3;
use crate::reduction;

use crate::constants::*;
use crate::rainbow_table::Node;

use colored::*;

pub fn execution(rainbow_table: &mut Vec<Node>) -> bool {

    let mut position_flag;

    let hash_flag = sha3::digest(FLAG);
    let mut reduce = reduction::reduce_truncate_xor(hash_flag, NONCE);

    for i in 0..NB_NODE {

        println!("{}","\n> Attack Node.. ".yellow());
        for j in NB_NODE-(i+1)..NB_NODE {
            
            let tmp = sha3::digest(&reduce.clone());
            reduce = reduction::reduce_truncate_xor(tmp, j+NONCE);
            
            if DEBUG {
                if j+1 == NB_NODE {
                    print!("{} (R{}) ", reduce, j);
                }
                else {
                    print!("{} (R{}) => ", reduce, j);
                }
            }
        }
        println!("search {}", reduce);

        position_flag = compare_end(rainbow_table, reduce.clone());
        if position_flag > 0 {
            if reverse(rainbow_table, hash_flag, position_flag) {
                return true;
            }
            else {
                continue;
            }
            
        } 
        else {
            println!("{}", "not find !".red());
        }
    }
    return false;
}

/*
*   Compare la value d'entrée à toute les valeurs "end" de la rainbowtable passé en entrée
*/
fn compare_end(rainbow_table: &mut Vec<Node>, value: String) -> u32 {

    for i in 0..NB_PASSWORD {
        if rainbow_table[i as usize].end == value {
            print!("{}", "find !".green());
            println!(" {} position {}", value, i);
            return i;
        }
    }
    return 0;
}

fn reverse(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32) -> bool {
    
    println!("{}", "> Recrate node..".yellow());
    println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    let mut reduce = rainbow_table[position_flag as usize].start.clone();
    let mut hash = sha3::digest(&reduce);

    for i in 0..NB_NODE+1 {

        if DEBUG { print!("{} => ", reduce); }

        if hash == hash_flag {
            println!("\n{} {}", "FLAG found !".green(), reduce);
            return true;
        }

        reduce = reduction::reduce_truncate_xor(hash, i+NONCE);
        hash = sha3::digest(&reduce);
    }
    println!("{}", "FLAG not found".red());
    return false;
}

pub fn print_hash(tab: [u8;32]) {
    for elt in tab {
        print!("{:0x}",elt);
    }
}