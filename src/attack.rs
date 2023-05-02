use crate::sha3::sha3;
use crate::reduction::reduction;

use crate::constants::*;
use crate::rainbow_table::Node;

use colored::*;

pub fn execution(rainbow_table: &mut Vec<Node>, flag: &str) -> bool {

    let mut position_flag;

    let mut tmp: [u8; 32];
    
    let hash_flag = sha3(flag);
    let mut reduce: String = String::from(flag);

    println!("On cherche : {}",flag.yellow());

    for i in 0..NB_NODE {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }


        for j in NB_NODE-(i+1)..NB_NODE {

            if j == NB_NODE-(i+1) {
                tmp = hash_flag;
                reduce = String::from(flag);
            }
            else {
                tmp = sha3(&reduce.clone());
            }
        
            if DEBUG {
                if j+1 == NB_NODE {
                    print!("{} (R{}) => ", reduce, j);
                }
                else {
                    print!("{} (R{}) => ", reduce, j);
                }
            }

            reduce = reduction(tmp, j+NONCE);
            
        }
        if DEBUG { print!("{}  ", reduce.red()); }

        if DEBUG { println!("search {}", reduce); }

        position_flag = compare_end(rainbow_table, reduce.clone());
        if position_flag != NB_PASSWORD {
            if reverse(rainbow_table, hash_flag, position_flag) {
            println!("{} == {} ce truc est {}",reduce,rainbow_table[position_flag as usize].end,reduce==rainbow_table[position_flag as usize].end); 
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
    return NB_PASSWORD;
}

fn reverse(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32) -> bool {
    
    if DEBUG {
        println!("{}", "> Recrate node..".yellow());
        println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    }

    let mut tmp: [u8; 32];
    let mut reduce: String = String::from("");

    let mut trouve = false;

    for i in 0..NB_NODE {

        if i == 0 {
            tmp = sha3(&rainbow_table[(position_flag)as usize].start);
        }
        else {
            tmp = sha3(&reduce.clone());
        }

        if DEBUG { print!("{} (R{}) => ", reduce,i); }

        if tmp == hash_flag {
            println!("{}", "FLAG found !".green());
            return true;
        }

        reduce = reduction(tmp, i+1+NONCE);
    }
    if DEBUG && !trouve { println!("{}", "FLAG not found".red()); }
    return trouve;
}

pub fn print_hash(tab: [u8;32]) {
    for elt in tab {
        print!("{:0x}",elt);
    }
}