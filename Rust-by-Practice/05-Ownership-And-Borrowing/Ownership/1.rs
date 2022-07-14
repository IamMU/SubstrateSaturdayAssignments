// Give
//
/*fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}*/

// My Solution(s)
fn main() {
    // Way One
    // Use a reference
    let x = String::from("hello, world");
    let y = &x;

    // Way Two
    // Use slicing to get the entire string
    let x = String::from("hello, world");
    let y = &x[..];

    // Way Three
    // Clone the string
    let x = String::from("hello, world");
    let y = x.clone();

    // I don't know about other ways

    println!("{},{}",x,y);
}
