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

//return used  below is for early return to avoid looping through the entire vector


struct GroceryItem {
    name: String,
    qty : i32
}

fn find_item(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name :"banana".to_owned(), qty : 5},
        GroceryItem { name :"eggs".to_owned(), qty : 25},
        GroceryItem { name :"bread".to_owned(), qty : 2},
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }
    None
}