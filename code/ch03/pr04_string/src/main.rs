fn main() {
    let mut text = String::from("Hello");
    
    text.push(' ');
    text += " World";
    
    println!("{text}");
}
