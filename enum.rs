enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn main() {
   let go = Direction::Up;

   match go {
       Direction::Left => println!("Went Left"),
       Direction::Right => println!("Went Right"),
       Direction::Down => println!("Went Down"),
       Direction::Up => println!("Went Up"),
   }
}


//enum revisited

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percents(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}