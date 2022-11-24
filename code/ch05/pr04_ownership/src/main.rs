fn main() {
    let s1 = String::from("Hello");
    let length = calculate_length(&s1);
    println!("String {} has length {}", s1, length);

    let mut s1 = String::from("Hello");
    add(&mut s1, " World!");
    println!("{s1}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn add(s: &mut String, value: &str) {
    s.push_str(value);
}
