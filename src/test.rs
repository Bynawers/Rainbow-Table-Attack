use crate::attack;

pub fn test(rainbow_table: &Vec<attack::Node>) {
    let mut deja_fait = Vec::<&str>::new();
    for elt in rainbow_table {
        if !contains(&elt.start,&deja_fait) {
            deja_fait.push(&elt.start);
        }
        if !contains(&elt.end,&deja_fait) {
            deja_fait.push(&elt.end);
        }
    }
    println!("pourcentage des mdp testés : {}",(deja_fait.len()as f32 / 36.0)*100.0);
}

fn contains(truc:&str,vector:&Vec<&str>) -> bool {
    for elt in vector {
        if &truc == elt {
            return true;
        }
    }
    false
}