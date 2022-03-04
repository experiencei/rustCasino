//option are used to catch Unprovided/provided value of variants
// if later provided use **some** if not use **None**
struct Customer {
    age : Option<i32>,
    email : String,
}

fn main() {
   let mark = Customer {
       age : Some(45) ,
       email : "mark@example.com".to_owned()
   },
   let quewxy = Customer {
       age : None ,
       email : "quewxy@example.com".to_owned()
   },
   match quewxy.email {
       Some(age) => println!("age is provided {:?}" , age),
       None => println!("age is not provided")
   }
}