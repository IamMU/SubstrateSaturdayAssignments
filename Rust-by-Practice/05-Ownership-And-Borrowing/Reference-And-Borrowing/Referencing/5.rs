// Given
//
/*fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = __;

    p.push_str("world");

    println!("Success!");
}*/

// My Solution
fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s; // Add mutable reference to s
    
    p.push_str("world");

    println!("Success!");
}
