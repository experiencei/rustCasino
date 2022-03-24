fn main() {
    let add = | a , b| a + b;

}

fn maybe_num() -> Option<i32> {
     
}

fn maybe_word() -> Option<String> {

}

fn main2() {
    let word_length = maybe_word()
             .map(|word| word.len())
             .map(|len| len * 2);

    let plus_one = maybe_num()
             .map(|num| num + 1);
}