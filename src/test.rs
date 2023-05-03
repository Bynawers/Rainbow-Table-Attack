#[cfg(test)]
mod tests {
    use crate::sha3::sha3;
    use std::fs::File;
    use std::io::Read;
    use serde_json::{from_str, Result};
    use crate::rainbow_table::Node;
    use crate::attack;
    use crate::constants;

    // les tests unitaires sur sha3 vérifient que le résultat obtenu avec notre fonction correspond bien au résultat que l'on
    // est cencé obtenir
    #[test]
    fn test_sha3_1() {
        let hash_abc:[u8;32] = [58,152,93,167,79,226,37,178,4,92,23,45,107,211,144,189,133,95,8,110,62,157,82,91,70,191,226,69,17,67,21,50];
        let res = sha3("abc");
        assert_eq!(hash_abc,res)
    }

    #[test]
    fn test_sha3_2() {
        let hash_vide:[u8;32] = [167,255,198,248,191,30,215,102,81,193,71,86,160,97,214,98,245,128,255,77,228,59,73,250,130,216,10,75,128,248,67,74];
        let res = sha3("");
        assert_eq!(res,hash_vide)
    }

    // test unitaire sur la fonction de réduction, vérifie que le résultat que l'on est cencé obtenir (vérifié à la main sur papier)
    // est bien celui que l'on obtient avec notre fonction
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

    // test unitaire sur l'attaque, on effectue une attaque sur une rainbow table dont on sait qu'elle contient le mot de passe 
    // recherché et on regarde si on trouve bien le mot de passe
    #[test]
    fn test_attack() {
        let file = File::open(format!("./data/RainbowTable_Test.json"));

        let mut contents = String::new();
        file.unwrap().read_to_string(&mut contents);

        let nodes: Vec<Node> = from_str(&contents).unwrap();
        let rainbow_table: Result<Vec<Node>> = Ok(nodes);
        
        let hash_ab = sha3("ab");

        let res = attack::execution_test(&mut rainbow_table.unwrap(), hash_ab);
        assert_eq!(res,true)
    }
    

}