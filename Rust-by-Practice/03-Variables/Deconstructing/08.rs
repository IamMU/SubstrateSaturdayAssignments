// We can use a pattern with let to destructure a tuple to seprarate variables.
//
// Given
//
// Fix the error below with least amount of modification
/*fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}*/

// My Solution
fn main() {
    let (mut x, y) = (1, 2); // We add mut keyword to make x mutable and hence be able to add 2 to it

    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
