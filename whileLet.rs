 fn main() {

    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop");
        data = None;
    }

    let numbers = vec![1, 2, 3, 4];
    let mut numbers_iter = numbers.iter();
    while let Some(i) = numbers_iter.next() {
          println("num = {:?}", i);
    }
    println!("Done!")
  }

  // .next() allow the looping to return an optional value as long as there are value to return
 