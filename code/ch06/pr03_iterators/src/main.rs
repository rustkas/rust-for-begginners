fn main() {
    let a = [1,2,3,4,5];

    for i in &a {
        println!("{i}");
    }
    println!("=================");

    let b = a.iter();

    for i in b {
        println!("{i}");
    }
    println!("=================");
    
    for i in a.iter().rev() {
        println!("{i}");
    }
}
