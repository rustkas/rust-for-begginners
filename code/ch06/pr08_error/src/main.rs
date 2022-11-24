#![allow(unused)]

use std::fs::File;

fn main() {
    // let vector = vec![1,2,3];
    // vector[100];

    // panic!("This should not have happend!");


    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt").unwrap();
    
}
