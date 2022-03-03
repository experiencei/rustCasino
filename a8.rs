enum Flavors {
   Banana,
   Vanilla,
}

struct DrinkFlavor {
   flavors : Flavors,
   fluid_oz : f64,
}

fn print_drink( juicy : DrinkFlavor) {
   
    match juicy.flavors {
       Flavor::Banana => println!("flavour : banana"),
       Flavor::Vanilla => println!("flavour : vanilla"),
    }

    println!("fluid_oz" , juicy.fluid_oz);
}

fn main() {
    let first_drink = DrinkFlavor {
        flavors: Flavor::Banana,
        fluid_oz: 0.9
    }

    print_drink(first_drink);

    let second_drink = DrinkFlavor {
        flavors: Flavor::Vanilla,
        fluid_oz: 8.10
    }

    print_drink(second_drink);
}