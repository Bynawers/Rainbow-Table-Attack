use rainbow_table::attack;
use rainbow_table::performance;

fn main() {
    let message = "Cryptographie";

    /*
    let performance = performance::perf_reduction(message, 500000, performance::Reduction::TruncateXor);

    match performance {
        Ok(value) => { println!("> Performance Reduction\n    collision: {:?}\n    time: {:?}", value.collision, value.time) },
        Err(e) => { println!("> Error : {:?}", e) }
    }*/

    attack::execution();
}