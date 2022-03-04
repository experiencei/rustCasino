fn print_many(msg: &str, count: i32) {

}

enum Mouse {
    LeftClick, 
    RightClick, 
    MiddleClick,
}

let num: i32 = 15;
let a: char = 'a';
let left_click: Mouse = Mouse::LeftClick;

//GENERICS

let numbers: Vec<i32> = vec![1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9]
let letters: Vec<char> = vec!["a", "b", "c", "d", "e", "f"]
let clicks:  Vec<Mouse> = vec![
    Mouse::LeftClick,
    Mouse::RightClick,
    Mouse::MiddleClick,
]