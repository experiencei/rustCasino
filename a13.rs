// struct Numbers {
//    number : 132
// }

// struct String {
//     letters : String
// }

// let numbers = vec![
//     Numbers { number : 10},
//     Numbers { number : 20 },
//     String { letters : "thirty"},
//     Numbers { number : 40},
// ];

// for num_let in numbers {
//      println!()
// }

// fn main() {

// }



fn main() {
    let my_numbers = vec![10, 20, 30, 40];
    for num in &my_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }

    println!("number of elements = {:?}", my_numbers.len());
}
