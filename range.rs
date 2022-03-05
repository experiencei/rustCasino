
fn main() {
  let range = 1..4;   // means not to include 4
  let range2 = 1..=4; // means to include 4

  for num in range2 {
      println!("{:?}", num);
  }

  for letter in 'a'..'z' {
      println!("{:?}", letter);
  }

}