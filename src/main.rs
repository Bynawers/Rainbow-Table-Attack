use sha3::{Sha3_256, Digest};

mod reduction;

const NONCE: u32 = 248820715;
const ROUND: u32 = 50000;

fn main() {
    let message = "Cryptographie";
    attack_round(message, ROUND);
}

fn attack_round(start: &str, nb_round: u32) {

    let mut hash = Sha3_256::digest(start.as_bytes());

    let mut password_xor: Vec<String> = Vec::new();
    let mut password_mod: Vec<String> = Vec::new();

    let mut collision_xor: usize = 0;
    let mut collision_mod: usize = 0;

    for i in 0..nb_round {
        password_xor.push(reduction::reduce_xor(hash.as_slice().try_into().unwrap(), i+NONCE));
        password_mod.push(reduction::reduce_mod(hash.as_slice().try_into().unwrap(), i+NONCE));
        println!("XOR : {}   |   MOD : {}", password_xor[i as usize], password_mod[i as usize]);

        hash = Sha3_256::digest(hash);
    }

    println!("sorting...");
    password_xor.sort();
    password_mod.sort();

    println!("search collisions...");
    collision_xor = collision(&password_xor);
    collision_mod = collision(&password_mod);

    /*
    for i in password_xor {
        println!("{}", i);
    }*/
    println!(" > {} ROUND :\n   XOR : {} collisions\n   MOD : {} collisions", ROUND, collision_xor, collision_mod);

}


fn collision<T: PartialEq>(vec: &[T]) -> usize {
    let mut count = 0;
    let len = vec.len();

    // Parcours du vecteur pour compter les éléments égaux consécutifs
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

