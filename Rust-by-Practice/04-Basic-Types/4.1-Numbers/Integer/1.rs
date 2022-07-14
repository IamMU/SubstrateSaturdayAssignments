// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for
// us.
//
// Given

// Remove something to make it work
/*fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}*/

// My Solution
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    //y = x; Remove this line because it gives a mismatched type error

    let z = 10; // Type of z? == u32
    
    println!("Success!");
}
