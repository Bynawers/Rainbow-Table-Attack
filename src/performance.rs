use std::time::{Instant, Duration};
use structopt::StructOpt;

use crate::{
    sha3::sha3,
    reduction::reduction,
    constants::*,
    rainbow_table::Node,
    attack,
    file::deserialize
};

use rayon::prelude::*;
use num_cpus;
use std::sync::{Arc, Mutex};

use indicatif::{ProgressBar, ProgressStyle,ProgressState};
use std::{fmt::Write};

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

pub fn perf_reduction() -> Performance {

    let mut password_reduce: Vec<String> = Vec::new();

    let mut hash = sha3(GENERATOR_RAINBOW_TABLE);
    let mut reduce;

    let start = Instant::now();

    for i in 0..NB_NODE*NB_PASSWORD {
        
        reduce = reduction(hash.as_slice().try_into().unwrap(), i+NONCE);
        if !contains(&reduce, &password_reduce) {
            password_reduce.push(reduce.clone());
        }
        hash = sha3(&reduce);
    }

    let end = Instant::now();
    let duration = end - start;

    println!("test {} / {} {}", password_reduce.len(), NB_NODE*NB_PASSWORD, (password_reduce.len() as f32/(NB_NODE*NB_PASSWORD) as f32)*100.0);

    return Performance { type_perf: Type::Reduction, percent: Some(0.0), collision: Some(0.0 as usize), time: duration };
}

pub fn perf_attack() -> Performance {

    let bar = ProgressBar::new(NB_PASSWORD_TOTAL as u64);

    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_bar:.magenta} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));

    let mut rainbow_table: Vec<Node> = deserialize().unwrap();

    let mut success = 0;
    let mut fail = 0;

    let start = Instant::now();
    let allpassword: Vec<String> = generate_passwords(&SIGMA, SIZE);
    println!("{}",allpassword.len());
    
    for i in 0..allpassword.len() {
        let hash = sha3(&allpassword[i]);
        if attack::execution(&mut rainbow_table, hash) {
            success += 1;
        }
        else {
            fail += 1;
        }
        bar.inc(1);
    }
    bar.finish();
    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::Attack, percent: Some(success as f32 / ((success + fail) as f32)*100.0), collision: None, time: duration };
}

pub fn perf_rainbow_table(rainbow_table: &Vec<Node>) -> Performance {

    let mut all_passw = Vec::<String>::new();

    let start = Instant::now();

    for elt in rainbow_table {
        let mut red = elt.start.clone();

        if !contains(&red,&all_passw) {
            all_passw.push(red.clone());
        }

        for i in 1..NB_NODE {
            let hash = sha3(&red);

            red = reduction(hash, i+NONCE);
            
            if !contains(&red, &all_passw) {
                all_passw.push(red.clone());
            }
        }
    }

    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::RainbowTable, percent: Some(all_passw.len() as f32 / NB_PASSWORD_TOTAL as f32 *100.0) ,collision: None, time: duration };
}

pub fn perf_para_rainbow_table(rainbow_table: &Vec<Node>) -> Performance {
    //println!("Thanu");
    let start = Instant::now();
    
    // Barre de chargement 
    let bar = ProgressBar::new(50);

    bar.set_style(ProgressStyle::with_template("{spinner:.magenta} {wide_bar:.magenta}")
        .unwrap());

    // Barre de chargement avec mémoire partagée.
    let bar_shared : Arc<Mutex<ProgressBar>> = Arc::new(Mutex::new(bar.clone()));

    let num_threads = num_cpus::get();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = NB_PASSWORD / num_threads as u32;
    
    let all_passw : Vec<String> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { NB_PASSWORD } else { start + slice };
                para_rainbow_test(start,end,rainbow_table, bar_shared.clone())
            }).flatten().collect()
    });
    let all_passw : Vec<String> = all_passw.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    let end = Instant::now();
    let duration = end - start;
    bar.finish_and_clear();
    println!("■ Fin du test des pourcentages de mot de passe.");

    return Performance { type_perf: Type::RainbowTable, percent: Some(all_passw.len() as f32 / NB_PASSWORD_TOTAL as f32 *100.0) ,collision: None, time: duration };
}

fn para_rainbow_test(startpassword : u32, endpassword: u32, rainbow_table: &Vec<Node>, bar: Arc<Mutex<ProgressBar>>) -> Vec<String> {
    //println!("{} start ,{} end ", startpassword ,endpassword);
    let mut all_passw: Vec<String> = Vec::<String>::new();
    let mut k: u32 = 1;
    for i in startpassword..endpassword {
        let mut red = rainbow_table[i as usize].start.clone();
        
        if !contains(&red,&all_passw) {
            all_passw.push(red.clone());
        }

        for j in 1..NB_NODE {
            let hash = sha3(&red);

            red = reduction(hash, j+NONCE);
            
            if !contains(&red, &all_passw) {
                all_passw.push(red.clone());
            }
        }
        if i == ((endpassword - startpassword) / 50) * k && k <= 50 {
            let barr = bar.lock().unwrap().inc((1)as u64);
            k += 1;
            drop(barr);
        }
    }
    all_passw
}


fn _collision<T: PartialEq>(vec: &[T]) -> u32 {
    let mut count = 0;
    let len = vec.len();

    let mut i = 0;
    while i < len {
        let mut j = i + 1;
        while j < len && vec[j] == vec[i] {
            count += 1;
            j += 1;
        }
        i = j;
    }

    count
}
fn contains(truc:&str, vector:&Vec<String>) -> bool {
    for elt in vector {
        if truc == elt {
            return true;
        }
    }
    return false;
}

fn generate_passwords(chars: &[char], n: u8) -> Vec<String> {
    if n == 0 {
        // Si la longueur est nulle, retourne un vecteur vide
        return Vec::new();
    } else if n == 1 {
        // Si la longueur est 1, retourne le tableau de caractères sous forme de vecteur de chaînes
        return chars.iter().map(|&c| c.to_string()).collect();
    } else {
        // Sinon, génère récursivement tous les mots de passe de longueur n-1
        let passwords = generate_passwords(chars, n - 1);
        // Crée un nouveau vecteur de chaînes pour stocker les mots de passe de longueur n
        let mut new_passwords = Vec::new();
        // Parcourt tous les mots de passe de longueur n-1 générés précédemment
        for password in passwords {
            // Ajoute chaque caractère possible à la fin du mot de passe
            for &c in chars {
                new_passwords.push(password.clone() + &c.to_string());
            }
        }
        // Retourne le vecteur de tous les mots de passe de longueur n
        return new_passwords;
    }
}