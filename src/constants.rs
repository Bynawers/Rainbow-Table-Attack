use lazy_static::lazy_static as set_constant;

// Attack
pub const GENERATOR_RAINBOW_TABLE: &str = "Crypto";
pub const FLAG: &str = "a8z";
pub const TEST:bool = false;

// Mot de Passe
pub const SIZE: u8 = 2;

pub const SIGMA_SIZE: u8 = 39;
pub const SIGMA: [char; 39] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z','!', '?','#'];


// Create RainbowTable
pub const NONCE: u32 = 248820715;

set_constant! {
    pub static ref NB_PASSWORD : u32 = ((((SIZE+1) as u32 * ((SIGMA_SIZE as u32).pow(SIZE as u32)))*50)as f32).sqrt() as u32;
    pub static ref NB_NODE: u32 = *NB_PASSWORD / 50;
}


// Debug
pub const DEBUG: bool = false;

// 36² = 1256       (100, 50) = 5000
// 36³ = 46 656     (500, 200) = 100 000
// 36⁴ = 1 679 616