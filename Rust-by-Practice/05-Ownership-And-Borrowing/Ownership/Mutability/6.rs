// Mutability can be changed when ownership is transferred
//
// Given
//
/*fn main() {
    let s = String::from("hello, ");

    // Modify this line only !
    let s1 = s;

    s1.push_str("world");

    println!("Success!");
}*/

// My Solution
fn main() {
    let mut s = String::from("hello, "); // Make var mutable
    
    // Make a mutable reference
    let s1 = &mut s;

    s1.push_str("world");

    println!("Success!");
}
