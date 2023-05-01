use std::time::{Instant, Duration};
use structopt::StructOpt;

use crate::sha3::sha3;
use crate::reduction::reduction;
use crate::constants::*;
use crate::rainbow_table::*;
use crate::attack;
use crate::file::*;

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

    let mut rainbow_table: Vec<Node> = deserialize().unwrap();

    let mut success = 0;
    let mut fail = 0;

    let start = Instant::now();

    for password in 100..(10 as u64).pow(SIZE as u32) {
        
        if attack::execution(&mut rainbow_table, &password.to_string()) {
            success += 1;
        }
        else {
            fail += 1;
        }
    }

    let end = Instant::now();
    let duration = end - start;

    return Performance { type_perf: Type::Attack, percent: Some((success / (success + fail)) as f32), collision: None, time: duration };
}

pub fn perf_rainbow_table(rainbow_table: &Vec<Node>) -> Performance {

    let mut all_passw = Vec::<String>::new();

    let start = Instant::now();

    for elt in rainbow_table {
        let mut red = elt.start.clone();

        if !contains(&red,&all_passw) {
            all_passw.push(String::from(red.clone()));
        }

        for i in 1..NB_NODE {
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


fn collision<T: PartialEq>(vec: &[T]) -> u32 {
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