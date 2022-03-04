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

// demonstration on string slice

struct Lineitem {
    name  : String,
    count : i36,
}


func main() {
  fn print_data(data: &str) {
      println!("{:?}", data);
  }


    let receipt = vec![
        Lineitem {
            name : "Ayeloja".to_owned(),
            count : 1,
        } ,
        Lineitem {
            name : String::from("Ibrahim"),
            count : 3,
        }
    
    ]
    
    for item in receipt {
        print_data(&item.name);
        println!("{:?}", item.count);
    }
}
