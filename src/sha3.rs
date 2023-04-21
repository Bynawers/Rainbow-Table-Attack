const RC : [u64; 24] = [0x8000000000000000,0x4101000000000000,0x5101000000000001,0x1000100000001,0xD101000000000000,0x8000000100000000,0x8101000100000001,
0x9001000000000001,0x5100000000000000,0x1100000000000000,0x9001000100000000,0x5000000100000000,0xD101000100000000,0xD100000000000001,
0x9101000000000001,0xC001000000000001,0x4001000000000001,0x100000000000001,0x5001000000000000,0x5000000100000001,0x8101000100000001,
0x101000000000001,0x8000000100000000,0x1001000100000001];

const R : [[i16;5] ; 5] = [[0, 36, 3, 41, 18], [1, 44, 10, 45, 2], 
[62, 6, 43, 15, 61], [28, 55, 25, 21, 56], [27, 20, 39, 8, 14]];

//modifié les caractère sont écrit à l'envers
fn padding(password : &str) -> u64 {
    //Padding
    let passacii = password.as_bytes();
    let mut padding64 : u64 = 0;
    let mut passwordlen = 0;
    for i in 0..passacii.len() {
        if i != 0 {
            padding64 = padding64 << 8;
        }
        padding64 = padding64 | passacii[i].reverse_bits() as u64;
        passwordlen += 8;
    }
    padding64 = padding64 << 3;
    padding64 = padding64 | 0b11; // 01 c'est le domaine pour tous les SHA-3 et le derniers 1 et le premier 1 du bourage.
    padding64 = padding64 << (64-passwordlen)-3;
    //println!("{}",format!("{:b}",padding64).len());
    padding64
}
//modifié le 1 devait être dans le bloc 17
fn bit_to_tab(m:u64) -> [[u64;5];5]{
    let big_tab:[[u64;5];5] = [[m,0b0,0b0,0b0,0b0],
    [0b0,0b0,0b0,0b1,0b0],
    [0b0,0b0,0b0,0b0,0b0],
    [0b0,0b0,0b0,0b0,0b0],
    [0b0,0b0,0b0,0b0,0b0]];
    //println!("{:?}",big_tab);
    return big_tab
    
}

fn kecak(mut a : [[u64;5];5] ) -> [[u64;5];5]{
    for i in 0..24 {
        a = round(a, RC[i]);
    }
    return a
}

fn rot(a1:u64,r_dec:i16) -> u64 {
    if r_dec == 0 {
        return a1
    }
    //println!("{} : a1 ",a1);
    let y = a1 >> r_dec;
    //println!("{} : décalage à droite.",format!("{:b}", y));
    let mask = (1 << r_dec) - 1;
    //println!("{} : mask.",format!("{:b}", mask));
    let last_bits = a1 & mask;
    //println!("{} : derniers bits.",format!("{:b}", last_bits));
    let mut rotate_bits = last_bits << (64 - r_dec);
    //println!("{} : rotation.",format!("{:b}", rotate_bits));
    //println!("{} : valeur donnée.",format!("{:b}", a1));
    rotate_bits |= y;
    //println!("{} : résultats.",format!("{:b}", rotate_bits));
    return rotate_bits
}

fn round(mut a : [[u64;5];5], v_rc : u64) -> [[u64;5];5] {
    // teta
    let mut c: [u64 ; 5] = [0,0,0,0,0];
    for x in 0..=4 {
        c[x] = a[x][0] ^ a[x][1] ^ a[x][2] ^ a[x][3] ^ a[x][4];
    }
    //println!("{:?} :a",a);
    let mut d: [u64 ; 5] = [0,0,0,0,0];
    
    for x in 0..=4 {
        d[x] = c[(x+5-1) % 5] ^ rot(c[(x+1)%5],1); // faire un shift sur [c[x+1]][1] pour qu'il boufe de i à i + R
    }
    //println!("{:?} :c",c);
    //println!("{:?}",d);
    for x in 0..=4{
        for y in 0..=4{
            a[x][y] = a[x][y] ^ d[x]
        }
    } 
    // p et pi
    let mut b: [[u64;5];5] = [[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0]];
    for x in 0..=4{
        for y in 0..=4{
            b[y][(((2*x) + (3*y)) % 5)] = rot(a[x][y],R[x][y]) // faire un shift sur [a[x][y]][R[x][y]] pour qu'il boufe de i à i + R
        }
    }
    //println!("{:?} : b",b); // j'ai pas verifier tous les elt du b
    for x in 0..=4{
        for y in 0..=4{
            a[x][y] = b[x][y] ^ ((!b[(x+1)%5][y]) & b[(x+2)%5][y])
        }
    }
    // T
    //println!("{:}",v_rc);
    a[0][0] = a[0][0] ^ v_rc;
    return a
}


fn extraction(big_tab:[[u64;5];5]) -> [u8;32] {
    let mut hash_octet : [u8;32] = [0;32];
    let mut c : String = String::new();
    //println!("{:?}",big_tab[0]);
    //let tab : [u64;4] = [0x5C19BAE5F247A44D,0x203AE8B4D6CB09BD,0xA1FA10767CB94ADA,0x62FD47A288C2A84C];
    //let value : u64 = 0x5C19BAE5F247A44D;
    let mut tab:[u64;4] = [0,0,0,0];
    for j in 0..4 {
        tab[j] = big_tab[j][0];
    }
     for j in 0..tab.len() {
        for i in 0..64/8 {
            let mask = 0xff << ((64-8)/8 - i)*8;
            //println!("{:b} : mask ",mask);
            let bits_value = tab[j] & mask;
            //println!("{:x}",bits_value);
            let octet_value : u8 = (bits_value >> ((64-8)/8 - i)*8) as u8;
            //println!("{:b}",octet_value);
            let octet_reverse: u8 = octet_value.reverse_bits();
            //println!("{}",i);
            hash_octet[i+ j*8] = octet_reverse;
            //println!("{:02x}",octet_reverse);
            c = c + format!("{:02x}",octet_reverse).as_str();
        }
    }
    
    //println!("{:?}",c);
    //println!("{:?}",hash_octet);
    //println!("{:b}",value.reverse_bits());
    //for elt in big_tab{}
    return hash_octet
}

pub fn sha3(password : &str) -> [u8;32] {
    extraction(kecak(bit_to_tab(padding(password))))
    /*let c = padding(password);
    let tab = bit_to_tab(c);
    //println!("{:?}",tab);
    let res = kecak(tab);
    let res = extraction(res);
    res*/
}