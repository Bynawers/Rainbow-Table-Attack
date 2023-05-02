use rainbow_table::attack;
use rainbow_table::performance;
use rainbow_table::rainbow_table::*;
use rainbow_table::test::*;
use rainbow_table::constants::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "attack" => {
                let mut rainbow_table: Vec<Node> = deserialize().unwrap();

                attack::execution(&mut rainbow_table,"aa");
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