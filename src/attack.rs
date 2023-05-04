use crate::sha3::sha3;
use crate::reduction::{reduction, reduction_test};

use crate::constants::*;
use crate::rainbow_table::Node;

use colored::*;

use rayon::prelude::*;
use num_cpus;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};



pub fn execution(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32]) -> bool {

    let mut position_flag;

    let mut tmp: [u8; 32];
    
    let mut reduce: String = String::new();

    //println!("On cherche : {}",hash_flag.yellow());

    /* Dans la boucle ci dessous, on va calculer reduce(hash) NB_NODES fois
     exemple :    -première itération :   reduce = reduce(hash_flag)
                    -deuxième itération :   reduce = reduce(hash(reduce(hash_flag)))
                    etc... jusqu'a NB_NODES itération
    Puis, à chaque itération, on compare le reduce avec toutes les fin de chaines de la rainbow table
    Si on trouve une fin de chaine = reduce, cela veut dire que le hashé recherché est peut-être dans la chaine
    On recalcule ensuite la chaine en repartant du première élément de la chaine, puis si on retombe sur le hashé recherché,
    on a retrouvé le mot de passe recheché et on renvoie le reduce précédent le hashé que l'on a retrouvé dans la chaine
*/
    for i in 0..NB_NODE {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }

        // dans cette boucle on caclcule NB_NODES-i+1 fois la fonction f = reduce(hash)
        for j in NB_NODE-(i+1)..NB_NODE {

            if j == NB_NODE-(i+1) {
                tmp = hash_flag;
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

        //ici on appelle la fonction compare_end qui renvoie la position de la chaine où le dernier élément = reduce
        // si aucune chaine ne correspond au reduce que l'on a, la fonction renvoie NB_PASSWORD (qui correspond a l'indice max + 1)
        position_flag = pool_search(rainbow_table, reduce.clone());
        if position_flag.len() != 0 {
            // on appelle ici la fonction reverse, qui recréé la chaine en repartant du premier élément de la chaine
            // cette foncion renvoie true si le hashé que l'on recherche est dans la chaine et false sinon
            for elt in position_flag {
                if reverse(rainbow_table, hash_flag, elt) {
                    return true;
                }
                else {
                    continue;
                }
            }
        } 
        else {
            if DEBUG { println!("{}", "not find !".red()); }
        }
    }
    if DEBUG { println!("{}", "FLAG not found".red());}
    return false;
}

fn pool_search(rainbow_table: &mut Vec<Node>, value: String) -> Vec<u32> {
    let num_threads = num_cpus::get();
    // Création d'une Pool de threads via la bibliothèque rayon
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = NB_PASSWORD / num_threads as u32;
    let allpositions: Vec<u32> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { NB_PASSWORD } else { start + slice };
                compare_end(rainbow_table.clone(), value.clone(), start, end)
            }).flatten().collect()
    });
    //println!("{}", allpositions.len());
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
/* 
fn compare_end(rainbow_table: &mut Vec<Node>, value: String) -> Vec<u32> {
    let mut allpositions : Vec<u32> = Vec::<u32>::new();
    for i in 0..NB_PASSWORD {
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
*/
//recréé la chaine à l'indice position_flag a partir du premier élément de la chaine et renvoie true si 
// hash flag est dans la chaine
fn reverse(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32], position_flag: u32) -> bool {
    
    if DEBUG {
        println!("{}", "> Recrate node..".yellow());
        println!("Position {} : ( first : {}, end: {} )", position_flag, rainbow_table[position_flag as usize].start, rainbow_table[position_flag as usize].end);
    }

    let mut tmp: [u8; 32];
    let mut reduce: String = String::from(rainbow_table[(position_flag)as usize].start.clone());

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
            if !TEST { println!("{} le mot de passe recherché est :  {}", "FLAG found !".green(),reduce.green()) };
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

/* les fonctions ci dessous sont identiques à celles qu'il y a au dessous à peu de choses près
les fonctions ci dessous ne font pas appel aux constantes SIZE, et sont appelées uniquement dans le cadre
des tests unitaires
 */

pub fn execution_test(rainbow_table: &mut Vec<Node>, hash_flag: [u8; 32]) -> bool {

    let mut position_flag;

    let mut tmp: [u8; 32];
    
    let mut reduce = String::new();

    //println!("On cherche : {}",flag.yellow());

    for i in 0..50 {

        if DEBUG { println!("{}","\n> Attack Node.. ".yellow()); }


        for j in 50-(i+1)..50 {

            if j == 50-(i+1) {
                tmp = hash_flag;
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

            reduce = reduction_test(tmp, j+NONCE);
            
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

        reduce = reduction_test(tmp, i+1+NONCE);
    }
    if DEBUG && !trouve { println!("{}", "FLAG not found".red()); }
    return trouve;
}
