fn print_it(data : &str) {
    println!("{:?}", data);
}

fn main() {
    println!("a string slice");
    let owned_string = "Owned string slice".to_owned();
    let another owned_string = String::from("Another string slice")

    println!(&owned_string);
    println!(&another owned_string);
}