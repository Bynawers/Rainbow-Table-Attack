use rainbow_table::sha3::sha3;
use rainbow_table::rainbowtable::rainbowtable;
fn main() {
    rainbowtable("abc");
    let mut tab_test: Vec<Node> = Vec::new();
    let hash = sha3("h7vljzt");
    for j in 0..nb_node {
        reduce = reduce_truncate_xor(hash.as_slice().try_into().unwrap(), j+constants::NONCE);
        hash = sha3(&reduce.clone());
        
        if j == 0 {
            node.start = reduce.clone();
        } else if j+1 == nb_node {
            node.end = reduce.clone();
        }
    }
    rainbow_table.push(node.clone());

}