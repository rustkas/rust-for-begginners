fn main() {
    let mut vector = vec![1,2,3,4,5];
    vector.push(10);

    println!("{:?}", vector);
    println!("");
    for i in &vector {
        println!("{i}");
    }
}
