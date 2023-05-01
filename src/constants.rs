// Attack
pub const GENERATOR_RAINBOW_TABLE: &str = "Crypto";
pub const FLAG: &str = "aaa";

// Create RainbowTable
pub const NONCE: u32 = 248820715;

pub const NB_PASSWORD: u32 = 500;
pub const NB_NODE: u32 = 200;

// Mot de Passe
pub const SIZE: u8 = 3;
pub const SIGMA_SIZE: u8 = 26;
pub const SIGMA: [char; SIGMA_SIZE as usize] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/*
pub const SIGMA: [char; SIGMA_SIZE as usize] = [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
pub const SIGMA_SIZE: u8 = 36; */

// Debug
pub const DEBUG: bool = true;

// 36² = 1256       (100, 50) = 5000
// 36³ = 46 656     (500, 200) = 100 000
// 36⁴ = 1 679 616


// 2 letttes: (60, 160) 95%
// 
//