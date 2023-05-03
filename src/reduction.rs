use crate::constants;

pub fn reduction(hash: [u8; 32], nonce: u32) -> String {
    return reduce_xor(hash, nonce);
}

pub fn reduction_test(hash: [u8; 32], nonce: u32) -> String {
    return reduce_xor_test(hash, nonce);
}

fn reduce_xor_test(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];
    for index in 0..2 {
        reduce[index] = hash[index] ^ nonce as u8;
    }
    let password = to_password_test(&reduce);
    password
}

fn reduce_xor_2(hash: [u8; 32], nonce: u32) -> String {
    let mut x = hash.clone();
    let size = constants::SIZE as usize;
    for i in 0..size {
        x[i] = x[i + 4 ] ^ nonce as u8; // mélanger avec le nonce
        x[i] = x[i] ^ x[31-i]; // appliquer une fonction non linéaire
    }
    let password = to_password(&x);
    password
}

pub fn reduce_xor_lvl2(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    //println!("{:?}", hash);
    for index in 0..8 {
        let mut bytes = [0u8; 4];
        bytes[..4].copy_from_slice(&hash[index*4..index*4+4]);
        let number = u32::from_be_bytes(bytes);
        let operation = number ^ nonce;
        let result = operation.to_be_bytes();
    
        reduce[index*4] = result[0]; 
        reduce[index*4 + 1] = result[1]; 
        reduce[index*4 + 2] = result[2]; 
        reduce[index*4 + 3] = result[3];
        //reduce[i] = hash[i] ^ nonce as u32;
    }

    let password = to_password(&reduce);
    password
}

pub fn reduce_xor_mod(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    for index in 0..32 {
        reduce[index] = hash[index] ^ nonce as u8;
        if nonce == 0 {
            reduce[index] = hash[index] % 1 as u8;
        }
        reduce[index] = hash[index] ^ nonce as u8;
    }

    let password = to_password(&reduce);
    password
}

pub fn reduce_xor(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];
    let size = constants::SIZE as usize;
    for index in 0..size {
        reduce[index] = hash[index] ^ nonce as u8;
    }

    let password = to_password(&reduce);
    password
}

pub fn reduce_mod(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    for i in 0..8 {
        let hash_slice = &hash[(i * 4)..((i+1) * 4)];
        let hash_value = u32::from_le_bytes([hash_slice[0], hash_slice[1], hash_slice[2], hash_slice[3]]);

        let hash_reduce = hash_value % nonce;
        let output = hash_reduce.to_le_bytes();

        reduce[(i * 4)..((i+1) * 4)].copy_from_slice(&output);
    }

    let password = to_password(&reduce);
    password
}

pub fn reduce_truncate(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    let start_value: usize = nonce as usize % 32;

    for index in start_value..32 {
        reduce[index] = hash[index];
    }
    for index in 0..start_value {
        reduce[index] = hash[index];
    }

    let password = to_password(&reduce);
    password
}

pub fn reduce_truncate_xor(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    let start_value: usize = nonce as usize % 32;

    for index in start_value..32 {
        reduce[index] = hash[index] ^ nonce as u8;
    }
    for index in 0..start_value {
        reduce[index] = hash[index] ^ nonce as u8;
    }

    let password = to_password(&reduce);
    password
}

fn to_password(bytes: &[u8; 32]) -> String {
    let mut password: String = String::from("");

    for i in 0..constants::SIZE {
        password.push(constants::SIGMA[((bytes[i as usize]) % constants::SIGMA_SIZE) as usize]);
    }
    password
}

fn to_password_test(bytes: &[u8; 32]) -> String {
    let mut password: String = String::from("");

    for i in 0..2 {
        password.push(constants::SIGMA[((bytes[i as usize]) % constants::SIGMA_SIZE) as usize]);
    }
    password
}
