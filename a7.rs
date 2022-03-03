// Topic: Working with an enum

enum Color {
    Red, 
    Green,
    Yellow,
    Black,
    Blue,
}

fn main() {
    let color = Color::Red;
    match color {
        Color::Red => println!("it's red"),
        Color::Green = println!("it's green"),
        Color::Yellow = println!("it's yellow"),
        Color::Black => println!("it's black"),
        Color::Blue = println!("it's blue"),
    }
}


enum Car {
    Toyota,
    Corrolla,
    Tesla,
    Samsung,
    Apple,
}

fn print_car(car: Car) {
     
    match car {
        Car::Toyota => println!("it's toyota"),
        Car::Corrolla => println!("it's corrolla"),
        Car::Apple => println!("it's apple"),
        Car::Samsung => println!("it's Samsung"),
        Car::Apple => println!("it's apple")
    }
}

fn main() {
    print_car(Car::Apple)
}