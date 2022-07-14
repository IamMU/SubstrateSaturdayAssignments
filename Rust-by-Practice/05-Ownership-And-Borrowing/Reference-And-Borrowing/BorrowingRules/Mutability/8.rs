// Error: Borrow an immutable object as mutable
//
// Given
//
/*fn main() {
    // Fix error by modifying this line
    let  s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}*/

// My Solution
fn main() {
    let mut s = String::from("hello, "); // Add mut to make s mutable

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
