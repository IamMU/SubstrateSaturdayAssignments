// Here, both [i32] and str are slice types, but directly using it will cause errors. You have to
// use the referrence of the slice instead: &[i32], &str
//
// Given
//
// Fix the errors, DON'T add new lines!
/*fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // Add ampersand to indicate that we are expecting a slice

    let s2: &str = "hello, world"; // Make str &str and remove as str

    println!("Success!");
}*/

// My Solution
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // Add ampersand to indicate that we are expecting a slice

    let s2: &str = "hello, world"; // Make str &str and remove as str

    println!("Success!");
}

