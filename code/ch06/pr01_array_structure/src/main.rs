#![allow(unused)]

fn main() {
    let num_list = [1,2,3,4,5];
    let num_list2: [i32;5] = [1,2,3,4,5];
    dbg!(num_list);
    dbg!(num_list2);

    let num_list: [i32;5] = [1,2,3,4,5];
    let same = ["Hi";10];

    dbg!(num_list);
    dbg!(same);

    println!("{}", num_list[0]);
}
