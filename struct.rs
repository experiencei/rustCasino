struct GroceryItem {
    stocks : i32,
    price : f64,
}

fn main () {
    let shoping = GroceryItem {
        stocks : 10,
        price : 45.5
    }

    println!(" stocks {:?}", shoping.stocks);
    println!(" price {:?}", shoping.price);

}