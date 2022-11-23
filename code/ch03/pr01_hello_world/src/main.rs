#![allow(unused)]

fn main() {
    let variable_name = 5;
    let blablala: u8;
    blablala = 5;

    let num1 = 5;
    let num2 = 10;

    let sum = num1 + num2;
    println!("{sum} {}", sum);

    let x = 5;

    let mut x = 5;
    x = 10;

    const PI: f32 = 3.14;

    let flag = 3 == 3;
    println!("{flag}");

    let flag = 3 <= 3;
    println!("{flag}");

    if 3 != 3 {
        println!("This is true");
    } else {
        println!("This is false");
    }

    let number = 0+1;

    if number == 0 {
        println!("Number is Zero");
    } else if number == 1 {
        println!("Number is One");
    } else {
        println!("I don't know number");
    }

    let flat = true;
    let number = if flag {10} else {2};
    println!("{number}");

}
