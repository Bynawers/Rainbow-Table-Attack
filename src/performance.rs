use std::time::{Instant, Duration};

use crate::sha3::sha3;
use crate::reduction;
use crate::constants::*;
use crate::rainbow_table::*;
use crate::attack;
use crate::file::*;

#[derive(Debug)] 
pub struct Performance {
    pub reduction_type: Option<Reduction>,
    pub collision: Option<usize>,
    pub time: Duration
}

#[derive(Debug)] 
pub enum Reduction {
    Xor,
    Modulo,
    Truncate,
    TruncateXor
}
#[derive(Debug)] 
pub enum Error {
    UnknowTypeError,
}

// XOR => 0 collisions en 500 000 et 10.7s
// MOD => 0 collisions en 500 000 et 10.8s
// TRUNCATE => 0 collisions en 500 000 et 10.7s
// TRUNCATE + XOR => 0 collisions en 500 000 et 10.9s
pub fn perf_reduction(nb_node: u32, type_reduction: Reduction) -> Result<Performance, Error> {

    let mut password_reduce: Vec<String> = Vec::new();

    let mut hash = sha3(GENERATOR_RAINBOW_TABLE);
    let mut reduce;

    let start = Instant::now();

    for i in 0..nb_node {
        match type_reduction {
            Reduction::Xor => {
                reduce = reduction::reduce_xor(hash.as_slice().try_into().unwrap(), i+NONCE);
            },
            Reduction::Modulo => {
                reduce = reduction::reduce_mod(hash.as_slice().try_into().unwrap(), i+NONCE);
            },
            Reduction::Truncate => {
                reduce = reduction::reduce_truncate(hash.as_slice().try_into().unwrap(), i+NONCE);
            },
            Reduction::TruncateXor => {
                reduce = reduction::reduce_truncate_xor(hash.as_slice().try_into().unwrap(), i+NONCE);
            },
            _ => {
                return Err(Error::UnknowTypeError);
            }
        }
        //println!("{}", reduce);
        password_reduce.push(reduce.clone());
        hash = sha3(&reduce);
    }

    let end = Instant::now();
    let duration = end - start;

    password_reduce.sort();

    let nb_collision = collision(&password_reduce);
    return Ok(Performance { reduction_type: None, collision: Some(nb_collision), time: duration });
}

pub fn perf_attack() -> Result<Performance, Error> {

    let mut rainbow_table: Vec<Node> = deserialize().unwrap();

    let mut success = 0;
    let mut fail = 0;

    let start = Instant::now();

    for password in 10..(10 as u64).pow(SIZE as u32) {
        //print!("{} => ",&password.to_string());
        attack::execution(&mut rainbow_table, &password.to_string());
    }

    let end = Instant::now();
    let duration = end - start;

    return Ok(Performance { reduction_type: None, collision: None, time: duration });
}

fn collision<T: PartialEq>(vec: &[T]) -> usize {
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