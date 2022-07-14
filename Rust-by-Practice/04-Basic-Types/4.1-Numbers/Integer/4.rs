// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for
// us.
//
// Given
//
// Fill the blanks to make it work
/*fn main() {
    assert_eq!(i8::MAX, __);
    assert_eq!(u8::MAX, __);

    println!("Success!");
}*/

// My Solution
fn main() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}
