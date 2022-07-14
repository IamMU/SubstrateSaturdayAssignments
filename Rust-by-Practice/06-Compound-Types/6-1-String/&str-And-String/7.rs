// Oposite to the seldom using of str, &str and String are used everywhere!
//
// &str can be converted to String in two ways
//
// Given
//
// Fix error with at least two solutions
/*fn main() {
    let s =  "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}*/

// My Solution
fn main() {
    // Way one
    // Use String::from
    let s =  String::from("hello, world");
    greetings(s);
    
    // Way two
    // to_string
    let s = "hello, world".to_string();
    greetings(s);
}

fn greetings(s: String) {
    println!("{}",s)
}

