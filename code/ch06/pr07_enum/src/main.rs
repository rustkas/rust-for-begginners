#![allow(unused)]

fn main() {
    #[derive(Debug)]
    enum Burger {
        Small,
        Medium,
        Large,
    }

    let burger_size = Burger::Large;
    
    print!("{:?}", burger_size);
    print!(" - ");
    let burger_size = {
        println!("{}", burger_size as u32);
        burger_size
    }
    
    match burger_size {
        Burger::Small => {
            println!("Small burger is ready");
        }
        Burger::Medium => {
            println!("Medium burger is ready");
        }
        Burger::Large => {
            println!("Large burger is ready");
        }
    }
}
