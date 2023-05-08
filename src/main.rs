use console::{style, Emoji};
use colored::*;
use structopt::StructOpt;
use std::time::{Instant};

use rainbow_table_attack::{
    sha3::sha3,
    attack,
    performance::*,
    rainbow_table::{ Node,pool },
    constants::*,
    file::*,
    interface,
};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍", "");
static SAVE: Emoji<'_, '_> = Emoji("🖿 ", "");
static ATTACK: Emoji<'_, '_> = Emoji("⚔️ ", "");
static DELETE: Emoji<'_, '_> = Emoji("🗑️ ", "");
static PERFORMANCE: Emoji<'_, '_> = Emoji("📈 ", "");

#[derive(StructOpt)]
#[structopt(name = "rainbow_table_attack", about = "A command-line for rainbow table attack")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "attack")]
    Attack {
    },
    #[structopt(name = "perf")]
    Performance {
        #[structopt(short = "t", long = "type", possible_values=&["attack", "reduction", "table"])]
        type_perf: Option<String>,
    },
    #[structopt(name = "test")]
    Test {
        
    },
    #[structopt(name = "delete")]
    Delete {
        #[structopt(short = "a", long = "all")]
        all: bool,
    },

    #[structopt(name = "table")]
    Table {
    },
    #[structopt(name = "start")]
    Start {
    },
    #[structopt(name = "sha3")]
    Sha3 {
        password: String,
    },
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Sha3 { password } => {
            let hashe = sha3(&password);
            for elt in hashe {
                print!("{:02x}",elt);
            }
        }
        Command::Attack { } => {
            println!("{} {} Generate Rainbow Table...", style("[1/3]").bold().dim(), LOOKING_GLASS);
            if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}.json", SIZE, NB_PASSWORD, NB_NODE)) {
                println!("Existing file found !");
            }
            else {
                create_table(NB_NODE, NB_PASSWORD, SIZE);
            }
            println!("{} {} Save File...", style("[2/3]").bold().dim(), SAVE);
            let mut rainbow_table: Vec<Node> = deserialize(&format!("RainbowTable_{}_{}_{}_{}.json", SIZE, 0, NB_PASSWORD, NB_NODE)).unwrap();

            println!("{} {} Attack...", style("[3/3]").bold().dim(), ATTACK);
            let result = attack::execution(&mut rainbow_table, sha3(FLAG), NB_NODE, NB_PASSWORD, SIZE); 

            match result {
                None => println!("{}", "Failed Attack!".red()),
                Some(password) => {
                    println!("{}", "Successful Attack!".green());
                    println!("> The password is : {}", password);
                }
            }
        }
        Command::Performance { type_perf } => {

            let performance: Option<Performance>;

            println!("{} {} Generate Rainbow Table...", style("[1/3]").bold().dim(), LOOKING_GLASS);
            create_table(NB_NODE, NB_PASSWORD, SIZE);

            println!("{} {} Save File...", style("[2/3]").bold().dim(), SAVE);
            let mut rainbow_table: Vec<Node> = deserialize(&format!("RainbowTable_{}_{}_{}_{}.json", SIZE, 0, NB_PASSWORD, NB_NODE)).unwrap();

            println!("{} {} Performance Testing...", style("[3/3]").bold().dim(), PERFORMANCE);

            match type_perf {
                None => performance = None,
                Some(value) => {
                    match value.as_str() {
                        "attack" => {
                            performance = Some(perf_attack(&mut rainbow_table, 10, NB_NODE, NB_PASSWORD, SIZE));
                        }
                        "table" => {
                            performance = Some(perf_para_rainbow_table(&rainbow_table, NB_NODE, NB_PASSWORD, SIZE));
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
            println!("> RainbowTable Password Total: {}", (NB_PASSWORD) * (NB_NODE));
            let start = Instant::now();
            let res = pool(NB_NODE, NB_PASSWORD, SIZE);
            serialize(&res, &format!("RainbowTable_{}_{}_{}_{}.json", SIZE, 0, NB_PASSWORD, NB_NODE)).unwrap();
            let end = Instant::now();
            let duration = end - start;
            println!("      time: {} seconds.", duration.as_secs_f32().to_string().purple());
            /*  Bordel ici 
            let start = Instant::now();
            create_table(NB_NODE, NB_PASSWORD, SIZE);
            let end = Instant::now();
            let duration = end - start;
            println!("      time: {:?}", duration)*/
        },
        Command::Table { } => {
            println!("{} {} Generate table of size {:?} NB_PASSWORDS and {} NB_NODES...", style("[1/1]").bold().dim(), DELETE,NB_PASSWORD,NB_NODE);

            create_table(NB_NODE, NB_PASSWORD, SIZE); 
        }
        Command::Delete { all } => {
            println!("{} {} Deleting...", style("[1/1]").bold().dim(), DELETE);

            if all {
                delete_all_file_in_directory("./data");
            }
            else {
                delete_file_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE, 0, NB_PASSWORD, NB_NODE));  
            }          
        }
        Command::Start { } => {
            let _ui = interface::main();
        }
    }
}

fn create_table(nb_node: u32, nb_password: u32, size: u8) {
    if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE, 0, NB_PASSWORD, NB_NODE)) {
        println!("RainbowTable already exist !");
    }
    else {
        let rainbow_table: Vec<Node> = pool(nb_node, nb_password, size);
        serialize(&rainbow_table, &format!("RainbowTable_{}_{}_{}_{}.json", SIZE, 0, NB_PASSWORD, NB_NODE)).unwrap();
    }
}