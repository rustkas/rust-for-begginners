struct Teplate {
    x: u32,
    y: i32,
    flag: bool,
}

fn main() {
    let temp = Teplate {
        x: 23,
        y: 34,
        flag: true,
    };
    println!("{} {} {}", temp.x, temp.y, temp.flag);
}
