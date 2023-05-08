use crate::constants;

// Renvoie le résultat de la fonction reduce_xor.
pub fn reduction(hash: [u8; 32], nonce: u32, size_char: u8) -> String {
    return reduce_xor(hash, nonce, size_char);
}

// Effectue un xor entre une nonce et un hashé sur les n premiers octets (n étant le nombre de caractères que l'on décide de prendre en compte).
fn reduce_xor(hash: [u8; 32], nonce: u32, size_char: u8) -> String {
    let mut reduce: [u8; 32] = [0; 32];

    for index in 0..32 {
        reduce[index] = hash[index] ^ nonce as u8;
    }

    let password = to_password(&reduce, size_char);
    password
}

// Extrait les n (n étant toujours le nombre de caractères) premiers octets d'un tableau de 32 octets et renvoie les
// éléments correspondants dans le tableau SIGMA qui contient les caractères que l'on a décidé de prendre en compte.
fn to_password(bytes: &[u8; 32], size: u8) -> String {
    let mut password: String = String::from("");

    for i in 0..size {
        password.push(constants::SIGMA[((bytes[i as usize]) % constants::SIGMA_SIZE) as usize]);
    }
    password
}

