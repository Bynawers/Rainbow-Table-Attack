use lazy_static::lazy_static as set_constant;

// Attack
pub const GENERATOR_RAINBOW_TABLE: &str = "Crypto";
pub const FLAG: &str = "p";
pub const TEST: bool = true;

// Create RainbowTable
pub const NONCE: u32 = 248820715;

set_constant! {
    pub static ref NB_PASSWORD : u32 = ((((SIZE as u64 + 1) * ((SIGMA_SIZE as u64).pow(SIZE as u32)))*50)as f32).sqrt() as u32;
    pub static ref NB_NODE: u32 = *NB_PASSWORD / 50;
}

// Mot de Passe
pub const SIZE: u8 = 4;

/*
pub const SIGMA_SIZE: u8 = 26;
pub const SIGMA: [char; SIGMA_SIZE as usize] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
*/

pub const SIGMA: [char; SIGMA_SIZE as usize] = [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '!', '?', '#'];
pub const SIGMA_SIZE: u8 = 39;

// Debug
pub const DEBUG: bool = false;