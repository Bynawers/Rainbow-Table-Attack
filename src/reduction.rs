const SIZE: u8 = 8;

const SIGMA_SIZE: u8 = 36;
const SIGMA: [char; 36] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn to_password(bytes: &[u8; 32]) -> String {
    let mut password: String = String::from("");

    for i in 0..SIZE {
        password.push(SIGMA[((bytes[i as usize]) % SIGMA_SIZE) as usize]);
    }
    password
}

pub fn reduce_xor(hash: [u8; 32], nounce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    for index in 0..32 {
        reduce[index] = hash[index] ^ nounce as u8;
    }

    let password = to_password(&reduce);
    password
}

pub fn reduce_mod(hash: [u8; 32], nounce: u32) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    for index in 0..32 {
        reduce[index] = hash[index] % nounce as u8;
    }

    //let password = to_password(&reduce);
    String::from("test")
}