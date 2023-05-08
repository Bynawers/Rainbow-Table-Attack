/*
#[cfg(test)]
mod tests {
    use crate::sha3::sha3;
    use std::fs::File;
    use std::io::Read;
    use serde_json::{from_str, Result};
    use crate::rainbow_table::Node;
    use crate::test;
    use crate::constants::*;

    // Les tests unitaires sur sha3 vérifient que le résultat obtenu avec notre fonction correspond bien au résultat que l'on
    // est cencé obtenir.
    #[test]
    fn test_sha3_1() {
        let hash_abc:[u8;32] = [58,152,93,167,79,226,37,178,4,92,23,45,107,211,144,189,133,95,8,110,62,157,82,91,70,191,226,69,17,67,21,50];
        let res = sha3("abc");
        assert_eq!(hash_abc,res)
    }

    #[test]
    fn test_sha3_2() {
        let hash_vide:[u8;32] = [167,255,198,248,191,30,215,102,81,193,71,86,160,97,214,98,245,128,255,77,228,59,73,250,130,216,10,75,128,248,67,74];
        let res = sha3("");
        assert_eq!(res,hash_vide)
    }

    // Test unitaire sur la fonction de réduction, vérifie que le résultat que l'on est cencé obtenir (vérifié à la main sur papier)
    // est bien celui que l'on obtient avec notre fonction.
    #[test]
    fn test_red_1() {
        let mut x = sha3("aa");
        for i in 0..2 {
            x[i] = x[i] ^ NONCE as u8;
        }
        let mut password: String = String::from("");

        for i in 0..2 {
            password.push(SIGMA[((x[i as usize]) % SIGMA_SIZE) as usize]);
        }
        assert_eq!("jh",password)
    }

    // Test unitaire sur l'attaque, on effectue une attaque sur une rainbow table dont on sait qu'elle contient le mot de passe 
    // recherché et on regarde si on trouve bien le mot de passe.
    #[test]
    fn test_attack() {
        let file = File::open(format!("./data/test/RainbowTable_Test.json"));

        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents);

        let nodes: Vec<Node> = from_str(&contents).unwrap();
        let rainbow_table: Result<Vec<Node>> = Ok(nodes);
        
        let res = execution_test(&mut rainbow_table.unwrap(), "ab", SIZE);
        assert_eq!(res, true)
    }   
}

/* Les fonctions ci dessous sont identiques à celles dans le fichier attack.rs à peu de choses près
les fonctions ci dessous ne font pas appel aux constantes SIZE,NB_PASSWORD ou NBNODES et sont appelées uniquement dans le cadre
des tests unitaires.
 */

/* Les fonctions ci dessous sont identiques à celles dans le fichier attack.rs à peu de choses près
les fonctions ci dessous ne font pas appel aux constantes SIZE,NB_PASSWORD ou NBNODES et sont appelées uniquement dans le cadre
des tests unitaires.
 */


pub fn execution_test(rainbow_table: &mut Vec<Node>, flag: &str, size: u8) -> bool {

    let mut position_flag;

    let mut reduce: String = String::from("");
    let mut tmp: [u8; 32];

    for i in 0..50 {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }

        for j in 50-(i+1)..50 {

            if j == 50-(i+1) {
                tmp = hash_flag;
            }
            else {
                tmp = sha3(&reduce.clone());
            }
        
            reduce = reduction_test(tmp, j+NONCE);
            
            if DEBUG {
                if j+1 == 50 {
                    print!("{} R({})", reduce, j);
                }
                else {
                    print!("{} R({}) => ", reduce, j);
                }
            }

            reduce = reduction(tmp, j+NONCE, size);
            
        }

        if DEBUG { println!("search {}", reduce); }

        position_flag = compare_end_test(rainbow_table, reduce.clone());
        if position_flag != 101 {
            if reverse_test(rainbow_table, hash_flag, position_flag, size) {
            println!("{} == {} ce truc est {}",reduce,rainbow_table[position_flag as usize].end,reduce==rainbow_table[position_flag as usize].end); 
                return true;
            }
        } 
        else {
            if DEBUG { println!("{}", "not find !".red()); }
        }
    }
    return None;
}

fn pool_search(rainbow_table: &mut Vec<Node>, value: String) -> Vec<u32> {
    let num_threads = num_cpus::get();
    
    // Création d'une Pool de threads via la bibliothèque rayon
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = 101 / num_threads as u32;

    let allpositions: Vec<u32> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { 101 } else { start + slice };
                compare_end(rainbow_table.clone(), value.clone(), start, end)
            }).flatten().collect()
    });

    let allpositions : Vec<u32> = allpositions.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    allpositions
}

/*
*   Compare la value d'entrée à toute les valeurs "end" de la rainbowtable passé en entrée
*/
fn compare_end(rainbow_table: Vec<Node>, value: String, start: u32, end: u32,
) -> Vec<u32> {
    let mut allpositions : Vec<u32> = Vec::<u32>::new();
    for i in start..end {
        if rainbow_table[i as usize].end == value {
            if DEBUG {
                print!("{}", "find !".green());
                println!(" {} position {}", value, i);
            }
            allpositions.push(i);
        }
    }
    allpositions
}

fn reverse_test(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32, size: u8) -> bool {
    
    if DEBUG {
        println!("{}", "> Recrate node..".yellow());
        println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    }

    let mut tmp: [u8; 32];
    let mut reduce: String = String::from("");

    let trouve = false;

    for i in 0..50 {

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

        reduce = reduction(tmp, i+1+NONCE, size);
    }
    if DEBUG { println!("{}", "FLAG not found".red()); }

    return None;
}


// Cette fonction effectue un xor entre une nonce et un hashé sur les 2 premiers octets.
fn reduction_test(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];
    for index in 0..2 {
        reduce[index] = hash[index] ^ nonce as u8;
    }
    let password = to_password_test(&reduce);
    password
}

// Cette fonction est identique à celle au dessus, mais elle etrait toujours 2 octets.
fn to_password_test(bytes: &[u8; 32]) -> String {
    let mut password: String = String::from("");

    for i in 0..2 {
        password.push(SIGMA[((bytes[i as usize]) % SIGMA_SIZE) as usize]);
    }
    password
}*/