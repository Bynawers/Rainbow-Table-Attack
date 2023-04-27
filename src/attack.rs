use crate::sha3::sha3;
use crate::reduction::reduction;

use crate::constants::*;
use crate::rainbow_table::Node;

use colored::*;

pub fn execution(rainbow_table: &mut Vec<Node>, flag: &str) -> bool {

    let mut position_flag;

    let hash_flag = sha3(flag);
    let mut reduce = reduction(hash_flag, NONCE);

    for i in 0..NB_NODE {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }
        for j in NB_NODE-(i+1)..NB_NODE {
            
            let tmp = sha3(&reduce.clone());
            reduce = reduction(tmp, j+NONCE);
            
            if DEBUG {
                if j+1 == NB_NODE {
                    print!("{} (R{}) ", reduce, j);
                }
                else {
                    print!("{} (R{}) => ", reduce, j);
                }
            }
        }
        if DEBUG { println!("search {}", reduce); }

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
            if DEBUG { println!("{}", "not find !".red()); }
        }
    }
    println!("{}", "FLAG not found".red());
    return false;
}

/*
*   Compare la value d'entrée à toute les valeurs "end" de la rainbowtable passé en entrée
*/
fn compare_end(rainbow_table: &mut Vec<Node>, value: String) -> u32 {

    for i in 0..NB_PASSWORD {
        if rainbow_table[i as usize].end == value {
            if DEBUG {
                print!("{}", "find !".green());
                println!(" {} position {}", value, i);
            }
            return i;
        }
    }
    return 0;
}

fn reverse(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32) -> bool {
    
    if DEBUG {
        println!("{}", "> Recrate node..".yellow());
        println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    }

    let mut reduce = rainbow_table[position_flag as usize].start.clone();
    let mut hash = sha3(&reduce);

    for i in 0..NB_NODE+1 {

        if DEBUG { print!("{} => ", reduce); }

        if hash == hash_flag {
            println!("{}", "FLAG found !".green());
            return true;
        }

        reduce = reduction(hash, i+NONCE);
        hash = sha3(&reduce);
    }
    if DEBUG { println!("{}", "FLAG not found".red()); }
    return false;
}

pub fn print_hash(tab: [u8;32]) {
    for elt in tab {
        print!("{:0x}",elt);
    }
}