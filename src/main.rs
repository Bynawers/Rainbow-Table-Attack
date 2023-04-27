use rainbow_table_attack::attack;
use rainbow_table_attack::performance;
use rainbow_table_attack::rainbow_table::*;
use rainbow_table_attack::test::*;
use rainbow_table_attack::sha3::sha3;
use rainbow_table_attack::constants::*;
use rainbow_table_attack::reduction::reduction;
use rainbow_table_attack::file::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "attack" => {
                create_table();
                let mut rainbow_table: Vec<Node> = deserialize().unwrap();

                attack::execution(&mut rainbow_table, FLAG); 
            }
            "perf" => { 
                println!("Performance...");
                let performance = performance::perf_reduction(500000, performance::Reduction::TruncateXor);

                match performance {
                    Ok(value) => { println!("> Performance Reduction\n    collision: {:?}\n    time: {:?}", value.collision, value.time) },
                    Err(e) => { println!("> Error : {:?}", e) }
                }
            }
            "table" => {
                create_table();
            }
            "test" => {
                create_table();
                let rainbow_table: Vec<Node> = deserialize().unwrap();

                let hash_flag = sha3("Crypto");
                let reduce = reduction(hash_flag, NONCE);
                //println!("{:?}", reduce);
                percent_of_password(&rainbow_table);
                //performance::perf_attack();
            }
            "delete" => {
                delete_file_in_directory("./data", &format!("RainbowTable_{}_{}_{}.json", SIZE, NB_PASSWORD, NB_NODE)).unwrap();
            }
            _ => { println!("bad args"); }
        }
    } else {
        panic!("too many arguments in prompt");
    }
}

fn create_table() {
    println!("> passwords: {} nodes: {}", NB_PASSWORD, NB_NODE);
    println!("> RainbowTable Password Total: {}", NB_PASSWORD * NB_NODE);
    println!("> Language Password Total: {}", (SIGMA_SIZE as u64).pow(SIZE as u32));
    println!("Create RainbowTable...");
    let rainbow_table: Vec<Node> = generate_table();
    serialize(&rainbow_table).unwrap();
}