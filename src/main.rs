use rainbow_table::attack;
use rainbow_table::performance;
use rainbow_table::rainbow_table::*;
use rainbow_table::verif::*;
use rainbow_table::constants::*;
use std::env;

use random_string::generate;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "attack" => {
                let mut compteur = 0;
                let nb_tests = 100;
                for _ in 0..nb_tests {
                    let mut rainbow_table: Vec<Node> = deserialize().unwrap();
                    let flag = generate(SIZE as usize,CHARSET);
                    if attack::execution(&mut rainbow_table,&flag) {
                        compteur = compteur+1;
                    }
                }
                println!("Sur {} réalisés, on a retrouvé {} mdp",nb_tests,compteur);
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
                println!("> passwords: {} nodes: {}", NB_PASSWORD, NB_NODE);
                println!("> RainbowTable Password Total: {}", NB_PASSWORD * NB_NODE);
                println!("> Language Password Total: {}", (SIGMA_SIZE as u64).pow(SIZE as u32));
                println!("Create RainbowTable...");
                let mut rainbow_table: Vec<Node> = generate_table();
                serialize(&rainbow_table).unwrap();
            }
            "test" => {
                let rainbow_table: Vec<Node> = deserialize().unwrap();

                test(&rainbow_table);
            }
            _ => { println!("bad args"); }
        }
    } else {
        panic!("too many arguments in prompt");
    }
}