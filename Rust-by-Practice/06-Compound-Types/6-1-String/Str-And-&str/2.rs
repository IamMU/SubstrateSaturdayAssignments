// We can only use str by boxed it, & can be used to convert Box<str> to &str
//
// Given
//
// Fix the error with at least two solutions
/*fn main() {
    let s: Box<str> =  "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}*/

// My Solution
fn main() {
    // Way one
    let s: Box<str> =  "hello, world".into();
    greetings(&s); // Add ampersand to turn it into &str
    
    // Way two
    let s: &str = "hello, world"; // Change data type to &str
    greetings(s);
}

fn greetings(s: &str) {
    println!("{}",s)
}

