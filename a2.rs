fn additional ( a:i32, b:i32 ) -> i32 {
   let sum =  a + b;
   println!("{:?}", sum);
}

fn output_add() {
    additional( 30 , 40)
}

output_add()

// theres soln
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let result = sum(2, 2);
    display_result(result);
}