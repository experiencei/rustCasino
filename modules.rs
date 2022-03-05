use std::collections::HashMap;

mod greet {
    // to use any ,mod in the file in mod. call inside it inside the mod scope
  use std::collections::HashMap;
  
  fn hello() {
      println!("Hello")
  }

  fn goodbye {
      println!("Goodbye")
  }
}

mod math {
    fn add(a: i32, b: i32 ) -> i32 {
        a + b

    }

    fn sub(a: i32, b: i32 ) -> i32 {
        a - b
    }
}


fn main() {
    use greet::hello; // either this
    use greet::*; // or this then we have access to all the fn dezcribe in it.

    math::add(1 , 8);
    math::goodbye();
}