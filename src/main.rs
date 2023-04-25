use rainbow_table::attack;
use rainbow_table::performance;
use rainbow_table::test;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match args[1].as_str() {
            "attack" => { 
                attack::execution(); 
            }
            "perf" => { 
                let performance = performance::perf_reduction(500000, performance::Reduction::TruncateXor);

                match performance {
                    Ok(value) => { println!("> Performance Reduction\n    collision: {:?}\n    time: {:?}", value.collision, value.time) },
                    Err(e) => { println!("> Error : {:?}", e) }
                }
            }
            "test" => {
              
            }
            _ => { println!("bad args"); }
        }
    } else {
        panic!("too many arguments in prompt");
    }


}