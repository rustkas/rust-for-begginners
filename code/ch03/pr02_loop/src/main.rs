fn main() {
    loop {
        println!("Hello!");
        break;
    }

    let mut number = 1;
    while number <= 5 {
        println!("The value of number is {}", number);
        number += 1;
    }
}
