use crate::sha3::sha3;
use crate::reduction::reduction;
use crate::constants::*;
use crate::rainbow_table::Node;

use rayon::prelude::*;
use num_cpus;

use colored::*;


/* Dans la boucle ci dessous, on va calculer reduce(hash) NB_NODES fois
*     exemple :    -première itération :   reduce = reduce(hash_flag)
*                    -deuxième itération :   reduce = reduce(hash(reduce(hash_flag)))
*                    etc... jusqu'a NB_NODES itération
*    Puis, à chaque itération, on compare le reduce avec toutes les fin de chaines de la rainbow table
*    Si on trouve une fin de chaine = reduce, cela veut dire que le hashé recherché est peut-être dans la chaine
*    On recalcule ensuite la chaine en repartant du première élément de la chaine, puis si on retombe sur le hashé recherché,
*    on a retrouvé le mot de passe recheché et on renvoie le reduce précédent le hashé que l'on a retrouvé dans la chaine
*/
pub fn execution(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32]) -> Option<String> {
    let mut position_flag;

    let mut reduce: String = String::from("");
    let mut tmp: [u8; 32];

    for i in 0..*NB_NODE {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }

        for j in *NB_NODE-(i+1)..*NB_NODE {

            if j == *NB_NODE-(i+1) {
                tmp = hash_flag;
            }
            else {
                tmp = sha3(&reduce.clone());
            }
        
            reduce = reduction(tmp, j+NONCE);
            
            if DEBUG {
                if j+1 == *NB_NODE {
                    print!("{} R({})", reduce, j);
                }
                else {
                    print!("{} R({}) => ", reduce, j);
                }
            }
        }

        if DEBUG { println!("search {}", reduce); }

        //ici on appelle la fonction compare_end qui renvoie la position de la chaine où le dernier élément = reduce
        // si aucune chaine ne correspond au reduce que l'on a, la fonction renvoie NB_PASSWORD (qui correspond a l'indice max + 1)
        position_flag = pool_search(rainbow_table, reduce.clone());
  
        if position_flag.len() != 0 {
            // on appelle ici la fonction reverse, qui recréé la chaine en repartant du premier élément de la chaine
            // cette foncion renvoie true si le hashé que l'on recherche est dans la chaine et false sinon
            for elt in position_flag {
                match reverse(rainbow_table, hash_flag, elt) {
                    None => continue,
                    Some(value) => return Some(value),
                }
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
    let slice = *NB_PASSWORD / num_threads as u32;

    let allpositions: Vec<u32> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { *NB_PASSWORD } else { start + slice };
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

//Recréé la chaine à l'indice position_flag a partir du premier élément de la chaine et renvoie true si 
// hash flag est dans la chaine.
fn reverse(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32) -> Option<String> {
    
    if DEBUG {
        println!("{}", "> Recrate node..".yellow());
        println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    }

    let mut tmp: [u8; 32];
    let mut reduce: String = String::from("");

    for i in 0..*NB_NODE+1 {

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


