// Destructuring tuple with pattern
// 
// Given
//
/*fn main() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let __ = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}*/

// My Solution
fn main() {
    let tup = (1, 6.4, "hello");

    let (x, z, y) = tup; // Just add the names so assert doesn't panic

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
