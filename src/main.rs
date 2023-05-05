use console::{style, Emoji};
use colored::*;
use structopt::StructOpt;
use std::time::{Instant};
use std::io;

use rainbow_table_attack::{
    sha3::sha3,
    attack,
    performance::*,
    rainbow_table::{ Node,pool },
    constants::{*, self},
    file::*,
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
    #[structopt(name = "test2")]
    Test2 {
        
    },
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Attack { } => {
            println!("{} {} Generate Rainbow Table...", style("[1/3]").bold().dim(), LOOKING_GLASS);
            if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE,0, *NB_PASSWORD, *NB_NODE)) {
                println!("Existing file found !");
            }
            else {
                create_table(0);
            }
            println!("{} {} Save File...", style("[2/3]").bold().dim(), SAVE);
            let mut rainbow_table: Vec<Node> = deserialize(0).unwrap();

            println!("{} {} Attack...", style("[3/3]").bold().dim(), ATTACK);
            let result = attack::execution(&mut rainbow_table, sha3(FLAG)); 

            let mut trouvé = false;

            match result {
                None => println!("{}", "Failed Attack!".red()),
                Some(password) => {
                    println!("{}", "Successful Attack!".green());
                    println!("> The password is : {}", password);
                    trouvé = true;
                }
            }
            
            if trouvé == false {
                println!("{}", "Would you like to try the attack on another table ? y/n".red());
                let mut entre = String::new();
                io::stdin().read_line(&mut entre).expect("An erroroccured with the input");
                if (entre.trim() == "y") || (entre.trim() == "Y") {
                    if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE,1, *NB_PASSWORD, *NB_NODE)) {
                        println!("Existing file found !");
                    } else {
                        create_table(1);
                    }
                    let mut rainbow_table: Vec<Node> = deserialize(1).unwrap();
                    println!("{} {} Attack...", style("[3/3]").bold().dim(), ATTACK);
                    let result2 = attack::execution(&mut rainbow_table, sha3(FLAG));
                    match result2 {
                                None => println!("{}", "Failed Attack!".red()),
                                Some(password) => {
                                    println!("{}", "Successful Attack!".green());
                                    println!("> The password is : {}", password);
                                    trouvé = true
                                }
                            }  
                }
            }

            if trouvé == false {
                println!("{}", "Would you like to try the attack on another table ? y/n".red());
                let mut entre = String::new();
                io::stdin().read_line(&mut entre).expect("An erroroccured with the input");
                if (entre.trim() == "y") || (entre.trim() == "Y") {
                    if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE,2, *NB_PASSWORD, *NB_NODE)) {
                        println!("Existing file found !");
                    } else {
                        create_table(2);
                    }
                    let mut rainbow_table: Vec<Node> = deserialize(2).unwrap();
                    println!("{} {} Attack...", style("[3/3]").bold().dim(), ATTACK);
                    let result2 = attack::execution(&mut rainbow_table, sha3(FLAG));
                    match result2 {
                                None => println!("{}", "Failed Attack!".red()),
                                Some(password) => {
                                    println!("{}", "Successful Attack!".green());
                                    println!("> The password is : {}", password);
                                }
                            }  
                }
            }
        }
        Command::Performance { type_perf } => {

            let performance: Option<Performance>;

            println!("{} {} Generate Rainbow Table...", style("[1/3]").bold().dim(), LOOKING_GLASS);
            create_table(0);

            println!("{} {} Save File...", style("[2/3]").bold().dim(), SAVE);
            let mut rainbow_table: Vec<Node> = deserialize(0).unwrap();

            println!("{} {} Performance Testing...", style("[3/3]").bold().dim(), PERFORMANCE);

            match type_perf {
                None => performance = None,
                Some(value) => {
                    match value.as_str() {
                        "attack" => {
                            performance = Some(perf_attack(&mut rainbow_table, 10));
                        }
                        "table" => {
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
            println!("> RainbowTable Password Total: {}", (*NB_PASSWORD) * (*NB_NODE));
            let start = Instant::now();
            let res = pool();
            serialize(&res,0).unwrap();
            let end = Instant::now();
            let duration = end - start;
            println!("      time: {} seconds.", duration.as_secs_f32().to_string().purple());
        },
        Command::Table { } => {
            println!("{} {} Generate table of size {:?} NB_PASSWORDS and {} NB_NODES...", style("[1/1]").bold().dim(), DELETE,*NB_PASSWORD,*NB_NODE);
            let mut compteur = 0;
            for n in 0..3 {
                if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE,n, *NB_PASSWORD, *NB_NODE)) {
                    compteur = compteur + 1;
                }
                //println!("{}",n);
            }
            println!("{}",compteur);
            if compteur == 0 {
                create_table(0);
            } else if compteur < 3 {
                create_table(compteur);
            } else {
                println!("Already 3 versions of this rainbow table exist, wich one would you like to replace ? 0/1/2");
                let mut entre = String::new();
                io::stdin().read_line(&mut entre).expect("An erroroccured with the input");
                let entre_int: u8 = entre.trim().parse().expect("Input was not an integer");
                if entre_int > 2 {
                    println!("L'entrée doit être comprise entre 0 et 2");
                } else {
                    delete_file_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE,entre_int, *NB_PASSWORD, *NB_NODE));
                    create_table(entre_int);
                }
            } 
        }
        Command::Delete { all } => {
            println!("{} {} Deleting...", style("[1/1]").bold().dim(), DELETE);

            if all {
                delete_all_file_in_directory("./data");
            }
            else {
                delete_file_in_directory("./data", &format!("RainbowTable_{}_{}_{}.json", SIZE, *NB_PASSWORD, *NB_NODE));  
            }          
        }
        Command::Test2 {} => {
            println!("{} {} constante", *NB_PASSWORD, *NB_NODE);         
            println!("{} {} constante", *NB_PASSWORD, *NB_NODE);         
        }
    }
}

fn create_table(i:u8) {
    if file_exists_in_directory("./data", &format!("RainbowTable_{}_{}_{}_{}.json", SIZE,i, *NB_PASSWORD, *NB_NODE)) {
        println!("RainbowTable already exist !");
    }
    else {
        let rainbow_table: Vec<Node> = pool();
        serialize(&rainbow_table,i).unwrap();
    }
}