// We can use String::from or to_string() to convert a &str to String
//
// Given
//
// Use two approaches to fix the error and without adding a new line
/*fn main() {
    let s =  "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}*/

// My Solution
fn main() {
    // Way one
    // Add a reference
    let s =  "hello, world".to_string();
    let s1: &str = &s; // Add & to make it &str
    
    // Way two
    // Make s an &str too
    let s = "hello, world";
    let s1: &str = s;

    println!("Success!");
}
