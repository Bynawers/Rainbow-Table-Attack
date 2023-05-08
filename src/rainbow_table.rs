use serde::{Serialize, Deserialize};
use rayon::prelude::*;
use num_cpus;
use std::sync::{Arc, Mutex};
use random_string::generate;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::{fmt::Write};

use crate::{
    sha3::sha3,
    reduction::reduction,
    constants::*
};

// Création d'une structure noeud contenant le début de la chaîne et la fin de la chaîne.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub start: String,
    pub end: String,
}

// Création de la rainbow_table avec de la parallélisation.
pub fn pool(nb_node:u32, nb_password: u32, size: u8) -> Vec<Node> {
    // On récupère le nombre de threads disponibles sur l'ordinateur.
    let num_threads = num_cpus::get();
    println!("Tu peux créer {} threads.", num_threads);

    // Création d'une barre du chargement.
    let bar = ProgressBar::new(50);
    bar.set_style(ProgressStyle::with_template("{spinner:.magenta} [{elapsed_precise}] {wide_bar:.magenta} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));
    
    // Création d'une Pool de threads via la bibliothèque rayon.
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = nb_password / num_threads as u32;
    
    // Variable qui a une mémoire partagé avec les threads.
    let starting_items_shared = Arc::new(Mutex::new(Vec::<String>::new()));
    let bar_shared : Arc<Mutex<ProgressBar>> = Arc::new(Mutex::new(bar.clone()));
    
    /* Initialisation des threads pour qu'ils exécutent la fonction generate_table sur une portion des mots de passe stockables dans la rainbow_table
    et que les portions de mots de passe générées par les threads seront assemblées dans une table unique.
    */
    let table: Vec<Node> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { nb_password } else { start + slice };
                generate_table(start, end, starting_items_shared.clone(), bar_shared.clone(), nb_node, size)
            })
            .flatten().collect()
    });
    bar_shared.lock().unwrap().inc(1);
    bar.finish_and_clear();
    println!("■ La génération de la RainbowTable est terminée.");
    table
}

// Création d'une portion de la Rainbow_table.
fn generate_table( startpassword: u32, endpassword: u32, starting_items_shared: Arc<Mutex<Vec<String>>>, bar: Arc<Mutex<ProgressBar>>, nb_node: u32, size: u8) -> Vec<Node> {

    let charset: String = SIGMA.iter().collect::<String>();
    let mut rainbow_table : Vec<Node> = vec![];
    let mut hash = sha3(GENERATOR_RAINBOW_TABLE);
    let mut reduce = generate(size as usize,&charset);
    let mut node = Node { 
        start: String::from(""), 
        end: String::from("") 
    };
    let mut k : u32 = 1;

    for i in startpassword..endpassword {
        for j in 0..nb_node {
            if j == 0 { 
                
                // Obtient l'accès à la variable, ainsi tous les autres threads sont mis en attente jusqu'à ce que la variable soit libérée.
                let mut starting_items = starting_items_shared.lock().unwrap();

                reduce = reduction(hash,j+NONCE, size);
                // Ici, on regarde si le mot de passe contenu dans reduce n'a pas déja été utilisé en début de chaine
                // si il a déja été utilisé, on génère aléatoirement un nouveau mot de passe jusqu'a en trouver un pas encore utilisé en début de chaine.
                while contains(reduce.to_string(),&mut starting_items) {
                    reduce = generate(size as usize,&charset);
                }
                // On défini le premier élément de la chaine avec le mot de passe obtenu à l'étape précédente.
                node.start = reduce.to_string();
                starting_items.push(reduce.to_string());
                
                //Libère la variable.
                drop(starting_items);

            } 
            // Si on est dans la dernière étape d'une chaine, on effectue un hashage puis un reduce sur le mot de passe que l'on
            // a actuellement, puis on définit la fin de chaine avec le mot de passe obtenu.
            else if j+1 == nb_node {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE, size);
                node.end = String::from(reduce.to_string());
            }

            // On effectue un hashage puis un reduce sur le mot de passe que l'on a.
            else {
                hash = sha3(&reduce);
                reduce = reduction(hash,j+NONCE, size);
            }

            // Progression de la barre de chargement.
            if i == ((endpassword - startpassword) / 50) * k && k <= 50 {
                let barr = bar.lock().unwrap().inc((1)as u64);
                k += 1;
                drop(barr);
            }
        }
        // Une fois la chaine fini (premier et dernier élément de la chaine définie), on ajoute la node au vecteur contenant la
        // rainbow table.
        rainbow_table.push(node.clone());
    }
    rainbow_table

}

// Cette fonction prend en argument un String et un vecteur de String et renvoie True si l'élément est dans le vecteur, et False sinon.
fn contains(elt:String, tab: &mut Vec::<String>) -> bool {
    for mdp in tab {
        if mdp == &elt {
            return true;
        }
    }
    return false;
}