use std::{collections::HashMap, io};

fn main() {
    let mut hash = HashMap::new();
    println!("Compose your employees plan");
    loop {
        println!("Enter a command like : add <Person> to <departement>");
        let mut order = String::new();
        io::stdin()
        .read_line( &mut order)
        .expect("error");

    let  order: &str = order.trim();
       let mut command =  order.split_whitespace();
   
       let  person = match command.nth(1) {
    Some(t)=>  t.to_owned() ,
    None => {
     println!("enter a command");
     continue;
    }
};

let departement = match command.nth(1) {
 Some(t)=>  t.to_owned(),
 None => {
  println!("enter a commandoooo");
  continue;
 }
};
    let persons = hash.entry(departement).or_insert(vec![]);
    persons.push(person);

    println!("{:?}",hash)
}
}
