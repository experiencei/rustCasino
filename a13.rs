struct Numbers {
   number : 132
}

struct String {
    letters : String
}

let numbers = vec![
    Numbers { number : 10},
    Numbers { number : 20 },
    String { letters : "thirty"},
    Numbers { number : 40},
];

for num_let in numbers {
     println!()
}

fn main() {

}