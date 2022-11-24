fn main() {
    let s1 = String::from("Hello");

    // This is shallow copy
    //    let s2 = s1;

    // This is deep copy
    let s2 = s1.clone();

    println!("{s2}");
    println!("{s1}");
}
