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

pub fn perf_reduction() -> Performance {

    let mut password_reduce: Vec<String> = Vec::new();

    let mut hash = sha3(GENERATOR_RAINBOW_TABLE);
    let mut reduce;

    let start = Instant::now();

    for i in 0..(*NB_NODE)*(*NB_PASSWORD) {
        
        reduce = reduction(hash.as_slice().try_into().unwrap(), i+NONCE);
        if !contains(&reduce, &password_reduce) {
            password_reduce.push(reduce.clone());
        }
        hash = sha3(&reduce);
    }

    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::Reduction, percent: Some(0.0), collision: Some(0.0 as usize), time: duration };
}

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

pub fn perf_rainbow_table(rainbow_table: &Vec<Node>) -> Performance {

    let mut all_passw = Vec::<String>::new();

    let start = Instant::now();

    for elt in rainbow_table {
        let mut red = elt.start.clone();

        if !contains(&red,&all_passw) {
            all_passw.push(String::from(red.clone()));
        }

        for i in 1..*NB_NODE {
            let hash = sha3(&red.clone());

            red = reduction(hash, i+NONCE);
            
            if !contains(&red, &all_passw) {
                all_passw.push(String::from(red.clone()));
            }
        }
    }

    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::RainbowTable, percent: Some(all_passw.len() as f32 / NB_PASSWORD_TOTAL as f32 *100.0) ,collision: None, time: duration };
}


pub fn perf_para_rainbow_table(rainbow_table: &Vec<Node>) -> Performance {
    let start = Instant::now();

    let bar = ProgressBar::new(*NB_PASSWORD as u64);

    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_bar} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));
    
    let num_threads = num_cpus::get();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let slice = *NB_PASSWORD / num_threads as u32;
    
    let all_passw : Vec<String> = pool.install(|| {
        (0..num_threads).into_par_iter()
            .map(|i| {
                let start = i as u32 * slice;
                let end = if i == num_threads - 1 { *NB_PASSWORD } else { start + slice };
                para_rainbow_test(start, end, rainbow_table, &bar)
            }).flatten().collect()
    });
    let all_passw : Vec<String> = all_passw.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::RainbowTable, percent: Some(all_passw.len() as f32 / NB_PASSWORD_TOTAL as f32 *100.0) ,collision: None, time: duration };
}
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

fn contains(truc:&str, vector:&Vec<String>) -> bool {
    for elt in vector {
        if truc == elt {
            return true;
        }
    }
    return false;
}