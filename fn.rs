fn add (a : i32, b : i32) -> i32 {
    a + b
}

let x = add(9 , 5);
let y = add(10 , 6);
let z = add( x , y); 

// print line macroff

println!("Hello");
println!("{:?}", x);

// conditional logic

let a = 99 ;

if a > 300 {
    println!("Huge number in the bucket")
} else if a < 100 {
    println!("Large number in the bucket")
} else {
    println!("small number in the bucket")
}

//  iteration in loop

let mut a = 0;

loop {
    if a == 5 {
       break;
    }
    println!("{:?}", a);
    a = a + 1;
}

//  iteration in while
let mut b = 0;
while a != 5 {
    println!("{:?}", b);
    b = b + 1;
}
