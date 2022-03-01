// working with match expressions

fn main () {
    let bool = true;

    match bool {
        true => println!("its True"),
        false => println!("its False"),
    }
}

// match expressions with integer

fn main () {
    let some_int = 1;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("its something else")
    }
}