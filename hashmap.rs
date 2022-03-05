let mut people = HashMap::new();
people.insert("Hassan" , 67);
people.insert("Hussan" , 57);
people.insert("Hessan" , 87);
people.remove("Hessan;");

match people.get("Hassan") {
    Some(age) => println!("age = {:?}" , age),
    None => println!("no age")
}


for (person, age) in people.iter() {
     println!("person = {:?} , age = {:?}", person, age);
}

for person in people.keys() {
    println!("person = {:?}", person);
}
for age in people.values() {
    println!("age = {:?}", age);
}