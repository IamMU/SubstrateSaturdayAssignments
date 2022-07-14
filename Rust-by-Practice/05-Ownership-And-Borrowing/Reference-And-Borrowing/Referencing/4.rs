// Given
//
// Fix error
/*fn main() {
    let mut s = String::from("hello, ");

    push_str(s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}*/

// My Solution
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s); // Add mutable reference to s

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}


