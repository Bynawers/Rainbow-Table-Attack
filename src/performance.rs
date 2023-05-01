use std::time::{Instant, Duration};
use crate::sha3::sha3;
use crate::reduction;

const NONCE: u32 = 248820715;

#[derive(Debug)] 
pub struct Performance {
    pub reduction_type: Reduction,
    pub collision: usize,
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
pub fn perf_reduction(message: &str, nb_node: u32, type_reduction: Reduction) -> Result<Performance, Error> {

    let mut password_reduce: Vec<String> = Vec::new();

    let mut hash = sha3(message);
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
    return Ok(Performance { reduction_type: type_reduction, collision: nb_collision, time: duration });
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