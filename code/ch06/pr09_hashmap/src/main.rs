use std::collections::HashMap;

fn main() {
    let mut scopes = HashMap::new();
    scopes.insert(String::from("Barselona"), 10);
    scopes.insert(String::from("Real Madrid"), 0);

    let team_name = String::from("Barselona");
    println!("{:?}", scopes.get(&team_name));

    println!("{:?}", scopes.get(&String::from("Bavaria")));


    for (key, value) in &scopes {
        println!("{}: {}", key, value);
    }
}
