// Given
//
// Fix all errors without adding newline
/*fn main() {
    let  s =  String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}*/

// My Solution
fn main() {
    let mut s =  String::from("hello"); // Make mutable
    s.push(','); 
    s.push_str(" world"); // Change to push_str because push only allows one char to be added
    s += "!"; // Remove to_string becuase it's already a &str

    println!("{}", s);
}

