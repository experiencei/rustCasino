struct Test {
    score : i32,
}


fn main() {

    let my_score = vec![
        Test { score: 66},
        Test { score: 77},
        Test { score: 88 },
        Test { score: 99 },
    ]
    
    for test in my_score {
        println!(" score = {:?}", test.score)
    }
    
}
