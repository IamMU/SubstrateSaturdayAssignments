// String type is defined in std and stored as a vector of bytes (Vec), but guranteed to always be
// a valid UTF-8 sequence. String is heap allocated, growable and not null terminated
//
// Given
//
// Fill the blank
/*fn main() {
    let mut s = __;
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}*/

// My Solution
fn main() {
    let mut s = String::from(""); // Make String type from an empty &str
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

