use sha3::{Sha3_256, Digest};

mod reduction;

fn main() {
    let message = "Cryptographie";
    attack_round(message, 10);
}

fn attack_round(start: &str, nb_round: u32) {

    let mut hash = Sha3_256::digest(start.as_bytes());

    for nounce in 0..nb_round {
        let password_xor = reduction::reduce_xor(hash.as_slice().try_into().unwrap(), nounce);
        let password_mod = reduction::reduce_mod(hash.as_slice().try_into().unwrap(), nounce);
        println!("Tour: {} =>    XOR : {}   |   MOD : {}", nounce, password_xor, password_mod);
        
        hash = Sha3_256::digest(hash);
    }
}