use crate::sha3::sha3;
use crate::reduction::reduction;
use crate::constants::*;
use crate::rainbow_table::Node;

use colored::*;

pub fn execution(rainbow_table: &mut Vec<Node>, flag: &str) -> Option<String> {

    let mut position_flag;

    let mut reduce: String = String::from("");
    let mut tmp: [u8; 32];
    let hash_flag = sha3(flag);

    for i in 0..NB_NODE {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }

        for j in NB_NODE-(i+1)..NB_NODE {

            if j == NB_NODE-(i+1) {
                tmp = hash_flag;
            }
            else {
                tmp = sha3(&reduce.clone());
            }
        
            reduce = reduction(tmp, j+NONCE);
            
            if DEBUG {
                if j+1 == NB_NODE {
                    print!("{} R({})", reduce, j);
                }
                else {
                    print!("{} R({}) => ", reduce, j);
                }
            }
        }

        if DEBUG { println!("search {}", reduce); }

        position_flag = compare_end(rainbow_table, reduce.clone());
        if position_flag != NB_PASSWORD {
            match reverse(rainbow_table, hash_flag, position_flag) {
                None => continue,
                Some(password) => { return Some(password); }
            }
            
        } 
        else {
            if DEBUG { println!("{}", "not find !".red()); }
        }
    }
    return None;
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
    return NB_PASSWORD;
}

fn reverse(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32) -> Option<String> {
    
    if DEBUG {
        println!("{}", "> Recrate node..".yellow());
        println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    }

    let mut tmp: [u8; 32];
    let mut reduce: String = String::from("");

    for i in 0..NB_NODE+1 {

        if i == 0 {
            tmp = sha3(&rainbow_table[(position_flag)as usize].start);
        }
        else {
            tmp = sha3(&reduce.clone());
        }

        if DEBUG { print!("{} => ", reduce); }

        if tmp == hash_flag {
            return Some(reduce);
        }

        reduce = reduction(tmp, i+NONCE+1);
    }
    if DEBUG { println!("{}", "FLAG not found".red()); }

    return None;
}
