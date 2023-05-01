use rainbow_table::rainbowtable::{generate_table,Node};
use rainbow_table::reduction::{reduce_truncate_xor};
use rainbow_table::sha3::sha3;

/*pub fn test(rainbow_table: Vec<Node>) {
    let mut all_passw = Vec::<String>::new();
    for elt in rainbow_table {
        let mut red = elt.start.clone();
        let startr = elt.start.clone();
        all_passw.push(String::from(&red));
        for i in 1..constants::NB_NODE {
            let hash = sha3(&red);
            red = reduce_truncate_xor(hash, i+constants::NONCE);
            all_passw.push(String::from(&red));
        }
        println!("start {}, end {}",startr, red);
    }
    //println!("{:?}",all_passw);
}*/


fn main() {
    let msg = "abc";
    sha3(msg);
    generate_table(msg);
    //assert_eq!(9675638965071249408,9675638965071249408);
}