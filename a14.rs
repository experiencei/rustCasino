struct Personality {
    age : i32,
    name : String,
    favourite : String,
}

fn print_name(data : &str) {
    println!("{:?}", data)
}

fn main() {

    let persons = vec![
        Personality { 
            age : 19,
            name : "faruk".to_owned(),
            favourite : String::from("yellow")
        },
        Personality { 
            age : 8,
            name : "twins".to_owned(),
            favourite : String::from("Green")
        },
        Personality { 
            age : 6,
            name : "prince".to_owned(),
            favourite : String::from("Blue")
        },
    ]


    for person in persons {
        if person.age <= 10 {
            print_name(&person.name);
            print_name(&person.favourite);
        }
    }

}