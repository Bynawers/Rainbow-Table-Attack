const RC : [u64; 24] = [0x8000000000000000,0x4101000000000000,0x5101000000000001,0x1000100000001,0xD101000000000000,0x8000000100000000,0x8101000100000001,
0x9001000000000001,0x5100000000000000,0x1100000000000000,0x9001000100000000,0x5000000100000000,0xD101000100000000,0xD100000000000001,
0x9101000000000001,0xC001000000000001,0x4001000000000001,0x100000000000001,0x5001000000000000,0x5000000100000001,0x8101000100000001,
0x101000000000001,0x8000000100000000,0x1001000100000001];

const R : [[i16;5] ; 5] = [[0, 36, 3, 41, 18], [1, 44, 10, 45, 2], 
[62, 6, 43, 15, 61], [28, 55, 25, 21, 56], [27, 20, 39, 8, 14]];

//Modifié les caractère sont écrit à l'envers.
fn padding(password : &str) -> u64 {
    //Padding
    let passacii = password.as_bytes();
    let mut padding64 : u64 = 0;
    let mut passwordlen = 0;
    for (i,elt) in passacii.iter().enumerate() {
        if i != 0 {
            padding64 <<= 8;
        }
        padding64 |= elt.reverse_bits() as u64;
        passwordlen += 8;
    }
    padding64 <<= 3;
    padding64 |= 0b11; // 01 c'est le domaine pour tous les SHA-3 et le derniers 1 et le premier 1 du bourage.
    padding64 <<= (64-passwordlen)-3;
    padding64
}
//Modifié le 1 devait être dans le bloc 17.
fn bit_to_tab(m:u64) -> [[u64;5];5]{
    let big_tab:[[u64;5];5] = [[m,0b0,0b0,0b0,0b0],
    [0b0,0b0,0b0,0b1,0b0],
    [0b0,0b0,0b0,0b0,0b0],
    [0b0,0b0,0b0,0b0,0b0],
    [0b0,0b0,0b0,0b0,0b0]];
    big_tab
    
}

// Appelle 24 fois la fonction round (commentée plus bas) en partant d'un tableau 5x5 contenant des entiers sur 64 bits
// et renvoie le tableau obtenu.
fn keccak(mut a : [[u64;5];5] ) -> [[u64;5];5]{
    for i in 0..24 {
        a = round(a, RC[i]);
    }
    a
}

// Effectue la fonction rotation, qui décale les bits d'un entier de 64 bits.
fn rot(a1:u64,r_dec:i16) -> u64 {
    if r_dec == 0 {
        return a1
    }
    let y = a1 >> r_dec;
    let mask = (1 << r_dec) - 1;
    let last_bits = a1 & mask;
    let mut rotate_bits = last_bits << (64 - r_dec);
    rotate_bits |= y;
    
    rotate_bits
}

// Appelle les fonctions iota,chi,thetha,rho_pi dans le bonne ordre et renvoie le tableau obtenu.
fn round(mut a : [[u64;5];5], v_rc : u64) -> [[u64;5];5] {
    let a_theta = theta(a);
    iota(chi(a_theta,rho_pi(a_theta)),v_rc)
}

//Effectue la fonction theta.
fn theta(mut a : [[u64;5];5]) -> [[u64;5];5]{
    let mut c: [u64 ; 5] = [0,0,0,0,0];
    for x in 0..=4 {
        c[x] = a[x][0] ^ a[x][1] ^ a[x][2] ^ a[x][3] ^ a[x][4];
    }
    let mut d: [u64 ; 5] = [0,0,0,0,0];
    
    for x in 0..=4 {
        d[x] = c[(x+5-1) % 5] ^ rot(c[(x+1)%5],1); 
    }
    for x in 0..=4{
        for y in 0..=4{
            a[x][y] = a[x][y] ^ d[x]
        }
    }
    a 
}

//Effectue les fonctions rho et pi.
fn rho_pi(a : [[u64;5];5]) -> [[u64;5];5]{
    let mut b: [[u64;5];5] = [[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0]];
    for x in 0..=4{
        for y in 0..=4{
            b[y][((2*x) + (3*y)) % 5] = rot(a[x][y],R[x][y]) // faire un shift sur [a[x][y]][R[x][y]] pour qu'il boufe de i à i + R
        }
    }
    b
}

//Effectue la fonction chi.
fn chi(mut a : [[u64;5];5], b : [[u64;5];5],) -> [[u64;5];5]{
    for x in 0..=4{
        for y in 0..=4{
            a[x][y] = b[x][y] ^ ((!b[(x+1)%5][y]) & b[(x+2)%5][y])
        }
    }
    a
}

//Effectue la fonction iota.
fn iota(mut a : [[u64;5];5], v_rc :u64) -> [[u64;5];5]{
    a[0][0] = a[0][0] ^ v_rc;
    a
}

//Effectue l'extraction du hashé (prend les 256 premiers bits du tableau obtenu).
fn extraction(big_tab:[[u64;5];5]) -> [u8;32] {
    let mut hash_octet : [u8;32] = [0;32];
    let mut tab:[u64;4] = [0,0,0,0];
    for j in 0..4 {
        tab[j] = big_tab[j][0];
    }
     for j in 0..tab.len() {
        for i in 0..64/8 {
            let mask = 0xff << ((64-8)/8 - i)*8;
            let bits_value = tab[j] & mask;
            let octet_value : u8 = (bits_value >> ((64-8)/8 - i)*8) as u8;
            let octet_reverse: u8 = octet_value.reverse_bits();
            hash_octet[i+ j*8] = octet_reverse;
        }
    }
    hash_octet
}

// Renvoie le hashé obtenu a partir de password.
pub fn sha3(password : &str) -> [u8;32] {
    extraction(keccak(bit_to_tab(padding(password))))
}