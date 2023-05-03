#[cfg(test)]
mod tests {
    use crate::sha3::sha3;
    use std::fs::File;
    use std::io::Read;
    use serde_json::{from_str, Result};
    use crate::rainbow_table::Node;
    use crate::attack;
    use crate::constants;

    #[test]
    fn test_sha3_1() {
        let hash_abc:[u8;32] = [58,152,93,167,79,226,37,178,4,92,23,45,107,211,144,189,133,95,8,110,62,157,82,91,70,191,226,69,17,67,21,50];
        let res = sha3("abc");
        assert_eq!(hash_abc,res)
    }

    #[test]
    fn test_sha3_2() {
        let hash_crypto:[u8;32] = [29,98,183,37,230,154,220,112,173,74,254,6,247,112,8,52,73,181,87,201,166,57,167,103,220,210,182,204,5,211,146,21];
        let res = sha3("crypto");
        assert_eq!(res,hash_crypto)
    }

    #[test]
    fn test_red_1() {
        let mut x = sha3("aa");
        for i in 0..2 {
            x[i] = x[i] ^ constants::NONCE as u8;
        }
        let mut password: String = String::from("");

        for i in 0..2 {
            password.push(constants::SIGMA[((x[i as usize]) % constants::SIGMA_SIZE) as usize]);
        }
        assert_eq!("yw",password)
    }

    #[test]
    fn test_attack() {
        let file = File::open(format!("./data/RainbowTable_Test.json"));

        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents);

        let nodes: Vec<Node> = from_str(&contents).unwrap();
        let rainbow_table: Result<Vec<Node>> = Ok(nodes);
        
        let res = attack::execution(&mut rainbow_table.unwrap(), "ab");
        assert_eq!(res,true)
    }
    
}