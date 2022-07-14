// Given
//
/*fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[__];

    assert_eq!(slice1, slice2);

    println!("Success!");
}*/

// My Solution
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2]; // Technicaly same as 0..2 but I didn't use 0 so...this should be valid

    assert_eq!(slice1, slice2);

    println!("Success!");
}
