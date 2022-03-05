fn main() {
   let numbers = vec![1, 2, 3, 4, 5, 6];

    let plus_one: Vec<> = numbers
       .iter()
       .map(|num| num + 1)
       .filter(|num| num <= 1)
       .collect();

    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| num <= 3)
        .collect();

    let find_me: Option<i32> = numbers
         .iter()
         .find(|num| num == 1);

    let count = numbers
         .iter()
         .count();

    let last: Option<i32> = numbers
         .iter()
         .last();

    let min: Option<i32> = numbers
         .iter()
         .min();

    let max: Option<i32> = numbers
         .iter()
         .max();

    let take: Vec<i32> = numbers
         .iter()
         .take(3)
         .collect();



}