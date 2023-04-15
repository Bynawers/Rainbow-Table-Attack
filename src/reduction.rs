const SIZE: u8 = 8;

const SIGMA_SIZE: u8 = 36;
const SIGMA: [char; 36] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

// a-z0-9 => 8^35 = 2 607 586 401 000 000 possibilitées
// 255 fonctions de réduction différentes (u8)
// 0 collisions en 500 000 répétitions sur 0,01% de 8^35


pub fn reduce_xor(hash: [u8; 32], nonce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    for index in 0..32 {
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

    for i in 0..SIZE {
        password.push(SIGMA[((bytes[i as usize]) % SIGMA_SIZE) as usize]);
    }
    password
}