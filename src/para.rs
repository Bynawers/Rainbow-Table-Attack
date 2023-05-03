use rayon::prelude::*;
use num_cpus;
use std::sync::{Arc, Mutex};

use random_string::generate;

use crate::{
    sha3::sha3,
    reduction::reduction,
    rainbow_table::Node,
    constants::*
};

// Création de la rainbow_table avec de la parallélisation
pub fn pool() -> Vec<Node> {
    // On récupère le nombre de threads disponibles sur l'ordinateur.
    let num_threads = num_cpus::get();
    println!("Tu peux créer {} threads.", num_threads);

    // Création d'une Pool de threads via la bibliothèque rayon
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = NB_PASSWORD / num_threads as u32;
    // Variable qui a une mémoire partagé avec les threads.
    let starting_items_shared = Arc::new(Mutex::new(Vec::<String>::new()));

    /* Initialisation des threads pour qu'ils exécutent la fonction generate_table sur une portion des mots de passe stockables dans la rainbow_table.
       Et que les portions de mots de passe générées par les threads seront assemblées dans une table unique.
    */
    let table: Vec<Node> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { NB_PASSWORD } else { start + slice };
                generate_table(start,end,starting_items_shared.clone())
            })
            .flatten().collect()
    });
    table
}
//pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyz";

// Création d'une portion de la Rainbow_table
fn generate_table(
    startpassword : u32,
    endpassword : u32, 
    starting_items_shared : Arc<Mutex<Vec<String>>>
) -> Vec<Node> {
    let charset: String = SIGMA.iter().collect::<String>();
    //println!("{} start ,{} end ", startpassword ,endpassword);
    let mut rainbow_table : Vec<Node> = vec![];
    let mut hash = sha3(GENERATOR_RAINBOW_TABLE);
    let mut reduce = generate(SIZE as usize,&charset);
    //let mut starting_items = Vec::<String>::new();
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };

    for _ in startpassword..endpassword {
        for j in 0..NB_NODE {
            if j == 0 { 
                
                // Obtient l'accès à la variable, ainsi tous les autres threads sont mis en attente jusqu'à ce que la variable soit libérée.
                let mut starting_items = starting_items_shared.lock().unwrap();

                reduce = reduction(hash,j+NONCE);

                while contains(reduce.to_string(),&mut starting_items) {
                    reduce = generate(SIZE as usize,&charset);
                }
                node.start = reduce.to_string();
                starting_items.push(reduce.to_string());
                
                //Libère la variable
                drop(starting_items);

            } 
            else if j+1 == NB_NODE {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE);
                node.end = String::from(reduce.to_string());
            } 
            else {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE);
                //print!("valeur de j : {}    ",j);
            }
        }
        rainbow_table.push(node.clone());
    }
    //println!("fin");
    rainbow_table

    //println!("{:?}",starting_items);
}

fn contains(elt:String, tab: &mut Vec::<String>) -> bool {
    for mdp in tab {
        if mdp == &elt {
            return true;
        }
    }
    return false;
}