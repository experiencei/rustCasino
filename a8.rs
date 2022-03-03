// enum Flavors {
//    Banana,
//    Vanilla,
// }

// struct DrinkFlavor {
//    flavors : Flavors,
//    fluid_oz : f64,
// }

// fn print_drink( juicy : DrinkFlavor) {
   
//     match juicy.flavors {
//        Flavor::Banana => println!("flavour : banana"),
//        Flavor::Vanilla => println!("flavour : vanilla"),
//     }

//     println!("fluid_oz" , juicy.fluid_oz);
// }

// fn main() {
//     let first_drink = DrinkFlavor {
//         flavors: Flavor::Banana,
//         fluid_oz: 0.9
//     }

//     print_drink(first_drink);

//     let second_drink = DrinkFlavor {
//         flavors: Flavor::Vanilla,
//         fluid_oz: 8.10
//     }

//     print_drink(second_drink);
// }

enum Flavor {
    Sparklings,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,

}

fn print_drink(drink: Drink) {
   
    match drink.flavor {
         Flavor::Sparklings => println!("flavor : sparklings")
         Flavor::Fruity => println!("flavor : fruity")
    }

    println!("fluid_oz" , drink.fluid_oz);
}

fn main() {
    let pass = Drink {
        flavor: Flavor::Sparklings,
        fluid_oz: 100.6
    }
    print_drink(pass);
}