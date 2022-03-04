struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey{
        q1: Some(45),
        q2: Some(true),
        q3: String::from("you get it right")
    }

    match response.q1 {
        Some(q) => println!("q1 : {:?}", q),
        None => println!("q1 : no response"),
    }
    match response.q2 {
        Some(q) => println!("q2 : {:?}", q),
        None => println!("q2 : no response"),
    }
    match response.q3 {
        Some(q) => println!("q3 : {:?}", q),
        None => println!("q3 : no response"),
    }
}