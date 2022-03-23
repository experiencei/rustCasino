fn main() {
// * Use a mutable integer variable

let mut v = 1;

// * Use a loop statement
loop {
 // * Print the variable within the loop statement
  println!("{:?}", v);

  if v == 4 { 
      println("{:?}", v);
      break
    }
// * Use break to exit the loop
v += 1
}
}

// repitition using while loop

fn main() {
    let mut x = 1;

    while x <= 3 {
        println!("{:?}", x);
        x += 1;
    }
}

fn main () {
    let mut v = 1;

    loop {


        println!("{:?}" , v);
        if v == 4 {
            break
        }

        v += 1;
    }
}