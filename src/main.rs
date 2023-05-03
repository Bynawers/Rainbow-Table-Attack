use rainbow_table_attack::{
    rainbow_table::generate_table,
    attack,
    performance::*,
    rainbow_table::*,
    constants::*,
    file::*,
    para::pool
};
use std::time::{Instant};
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "combat", about = "A command-line simulation of combat")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "attack")]
    Attack {
        #[structopt(short = "s", long = "save")]
        save: bool,
    },
    #[structopt(name = "perf")]
    Performance {
        #[structopt(short = "t", long = "type", possible_values=&["attack", "reduction", "table"])]
        type_perf: Option<String>,
    },
    #[structopt(name = "test")]
    Test {
        
    },
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Attack { save } => {
            println!("Attack... with option ? : {}", save);
            if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}.json", SIZE, NB_PASSWORD, NB_NODE)) {
                println!("Existing file found !");
            }
            else {
                create_table();
            }
            let mut rainbow_table: Vec<Node> = deserialize().unwrap();

            attack::execution(&mut rainbow_table, FLAG); 
        }
        Command::Performance { type_perf } => {
            println!("Performance...");
            let performance: Option<Performance>;

            match type_perf {
                None => performance = None,
                Some(value) => {
                    match value.as_str() {
                        "reduction" => {
                            performance = Some(perf_reduction());
                        },
                        "attack" => {
                            performance = Some(perf_attack());
                        }
                        "table" => {
                            create_table();
                            let rainbow_table: Vec<Node> = deserialize().unwrap();
                            performance = Some(perf_para_rainbow_table(&rainbow_table));
                        }
                        _ => performance = None
                    }
                }
            }
            match performance {
                Some(value) => { 
                    println!("> Performance {:?}", value.type_perf);
                    println!("      time: {:?}", value.time);
                    println!("      percent test: {:?}%", value.percent.unwrap());
                    },
                None => ()
            }
        }
        Command::Test { } => {
            println!("Parallel Testing ..");
            println!("> RainbowTable Password Total: {}", NB_PASSWORD * NB_NODE);
            let start = Instant::now();
            let res = pool();
            serialize(&res).unwrap();
            let end = Instant::now();
            let duration = end - start;
            println!("      time: {} seconds.", duration.as_secs_f32().to_string().purple());
            /*  Bordel ici 
            let start = Instant::now();
            create_table();
            let end = Instant::now();
            let duration = end - start;
            println!("      time: {:?}", duration)*/
        }
    }
}

fn create_table() {
    println!("> Passwords: {} Nodes: {}", NB_PASSWORD, NB_NODE);
    println!("> RainbowTable Password Total: {}", NB_PASSWORD * NB_NODE);
    println!("> Language Password Total: {}", (SIGMA_SIZE as u64).pow(SIZE as u32));
    /*if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}.json", SIZE, NB_PASSWORD, NB_NODE)) {
        println!("RainbowTable already exist !");
    }
    else {
        println!("Create RainbowTable...");
        let rainbow_table: Vec<Node> = generate_table();
        serialize(&rainbow_table).unwrap();
    }*/
    println!("Create RainbowTable...");
    let rainbow_table: Vec<Node> = pool();
    serialize(&rainbow_table).unwrap();
}