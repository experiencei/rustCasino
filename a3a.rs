fn main() {
        let message = true;
    if message == true {
        println!("hello")
    } else {
        println!("goodbye")
    }
}


// control flow with else if

fn main() {
    let n = 7;
    if n > 5 {
        println!(">5");
    } else if n < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}