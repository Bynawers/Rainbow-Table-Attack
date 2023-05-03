use crate::{
    sha3::sha3,
    reduction::reduction,
    rainbow_table::Node,
    constants::*
};

use colored::*;

#[cfg(test)]
mod tests {
    use crate::sha3::sha3;
    use std::fs::File;
    use std::io::Read;
    use serde_json::{from_str, Result};
    use crate::rainbow_table::Node;
    use crate::test::*;
    use crate::constants;

    #[test]
    fn test_sha3_1() {
        let hash_abc:[u8;32] = [58,152,93,167,79,226,37,178,4,92,23,45,107,211,144,189,133,95,8,110,62,157,82,91,70,191,226,69,17,67,21,50];
        let res = sha3("abc");
        assert_eq!(hash_abc,res)
    }

    #[test]
    fn test_sha3_2() {
        let hash_crypto:[u8;32] = [29,98,183,37,230,154,220,112,173,74,254,6,247,112,8,52,73,181,87,201,166,57,167,103,220,210,182,204,5,211,146,21];
        let res = sha3("crypto");
        assert_eq!(res,hash_crypto)
    }

    #[test]
    fn test_red_1() {
        let mut x = sha3("aa");
        for i in 0..2 {
            x[i] = x[i] ^ constants::NONCE as u8;
        }
        let mut password: String = String::from("");

        for i in 0..2 {
            password.push(constants::SIGMA[((x[i as usize]) % constants::SIGMA_SIZE) as usize]);
        }
        assert_eq!("yw", password)
    }

    #[test]
    fn test_attack() {
        let file = File::open(format!("./data/test/RainbowTable_Test.json"));

        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents);

        let nodes: Vec<Node> = from_str(&contents).unwrap();
        let rainbow_table: Result<Vec<Node>> = Ok(nodes);
        
        let res = execution_test(&mut rainbow_table.unwrap(), "ab");
        assert_eq!(res, true)
    }   
}


pub fn execution_test(rainbow_table: &mut Vec<Node>, flag: &str) -> bool {

    let mut position_flag;

    let mut tmp: [u8; 32];
    
    let hash_flag = sha3(flag);
    let mut reduce: String = String::from(flag);

    println!("On cherche : {}",flag.yellow());

    for i in 0..50 {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }


        for j in 50-(i+1)..50 {

            if j == 50-(i+1) {
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

        position_flag = compare_end_test(rainbow_table, reduce.clone());
        if position_flag != 101 {
            if reverse_test(rainbow_table, hash_flag, position_flag) {
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
fn compare_end_test(rainbow_table: &mut Vec<Node>, value: String) -> u32 {

    for i in 0..101 {
        if rainbow_table[i as usize].end == value {
            if DEBUG {
                print!("{}", "find !".green());
                println!(" {} position {}", value, i);
            }
            return i;
        }
    }
    return 101;
}

fn reverse_test(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32) -> bool {
    
    if DEBUG {
        println!("{}", "> Recrate node..".yellow());
        println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    }

    let mut tmp: [u8; 32];
    let mut reduce: String = String::from("");

    let mut trouve = false;

    for i in 0..50 {

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