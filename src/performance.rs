use crate::sha3::sha3;
use crate::reduction::reduction;
use crate::constants::*;
use crate::rainbow_table::*;
use crate::attack::*;

use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::time::{Instant, Duration};
use structopt::StructOpt;
use std::{fmt::Write};
use rayon::prelude::*;
use num_cpus;

#[derive(Debug)] 
pub struct Performance {
    pub type_perf: Type,
    pub percent: Option<f32>,
    pub collision: Option<usize>,
    pub time: Duration
}

#[derive(StructOpt)]
#[derive(Debug)] 
pub enum Type {
    Reduction,
    Attack,
    RainbowTable
}
#[derive(Debug)] 
pub enum Error {
    UnknowTypeError,
}

const NB_PASSWORD_TOTAL: u64 = (SIGMA_SIZE as u64).pow(SIZE as u32);

// Cette fonction appelle la fonction attack dans le fichier attack.rs et affiche le temps que celle-ci met à s'exécuter.
pub fn perf_attack(rainbow_table: &mut Vec<Node>, nb_test: u32) -> Performance {

    let bar = ProgressBar::new(nb_test as u64);

    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_bar} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));

    let mut password: String = "a".repeat(SIZE as usize);
    let mut success = 0;
    let mut fail = 0;

    let start = Instant::now();

    for _ in 0..nb_test {
        password = increment_string(&password);
        
        match execution(rainbow_table, sha3(&password.to_string())) {
            None => fail += 1,
            Some(_) => success += 1
        }

        bar.inc(1);
    }
    bar.finish();

    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::Attack, percent: Some(success as f32 / ((success + fail) as f32)*100.0), collision: None, time: duration };
}

/* 
*   Créer plusieurs threads pour permettre la parallélisation du test."
*/
pub fn perf_para_rainbow_table(rainbow_table: &Vec<Node>) -> Performance {
    // Le timer a été initialisé.
    let start = Instant::now();

    // Initialisation d'une barre de progression.
    let bar = ProgressBar::new(*NB_PASSWORD as u64);
    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_bar} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));
    
    // Récupération du nombre max de threads disponible et création du pool de thread.
    let num_threads = num_cpus::get();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = *NB_PASSWORD / num_threads as u32;
    
    // Initialisation du pool de threads et récupération des données dans une variable.
    let all_passw : Vec<String> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { *NB_PASSWORD } else { start + slice };
                para_rainbow_test(start, end, rainbow_table, &bar)
            }).flatten().collect()
    });

    // On récupère les éléments distincts du vecteur.
    let all_passw : Vec<String> = all_passw.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    
    // Fin du timer.
    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::RainbowTable, percent: Some(all_passw.len() as f32 / NB_PASSWORD_TOTAL as f32 *100.0) ,collision: None, time: duration };
}

/* 
    Cette fonction prend en argument une rainbow table. On part des premiers noeuds de chaque ligne de la rainbow table et on recréer les lignes à partir
    de ces derniers. A chaque fois que l'on tombe sur une chaîne de caractères que l'on a pas encore croisé, on l'ajoute à un vecteur.
    Une fois le processus terminé, on divise SIGMA _SIZE(le nombre de carcatère que l'on décide de prendre en comtpe)^SIZE(la taille des chaînes de caractères)
    par la taille du vecteur. On multiplie le résultat par 100 ce qui nous donne le pourcentage de mots de passes que l'on a testé. On affiche aussi le temps que cette
    fonction a pris pour s'exécuter.
 */
fn para_rainbow_test(startpassword : u32, endpassword: u32, rainbow_table: &Vec<Node>, bar: &ProgressBar) -> Vec<String> {
    
    let mut all_passw: Vec<String> = Vec::<String>::new();

    for i in startpassword..endpassword {
        let mut red = rainbow_table[i as usize].start.clone();
        
        if !contains(&red,&all_passw) {
            all_passw.push(red.clone());
        }

        for j in 1..*NB_NODE {
            let hash = sha3(&red);

            red = reduction(hash, j+NONCE);
            
            if !contains(&red, &all_passw) {
                all_passw.push(red.clone());
            }
        }
        bar.inc(1);
    }
    all_passw
}

fn increment_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut carry = true;
    for i in (0..chars.len()).rev() {
        if carry {
            if chars[i] == 'z' {
                chars[i] = 'a';
                carry = true;
            } else {
                chars[i] = (chars[i] as u8 + 1) as char;
                carry = false;
            }
        }
    }
    if carry {
        chars.insert(0, 'a');
    }
    chars.into_iter().collect()
}

// Cette fonction renvoie True si truc est dans le vecteur vector, et False sinon.
fn contains(truc:&str, vector:&Vec<String>) -> bool {
    for elt in vector {
        if truc == elt {
            return true;
        }
    }
    return false;
}