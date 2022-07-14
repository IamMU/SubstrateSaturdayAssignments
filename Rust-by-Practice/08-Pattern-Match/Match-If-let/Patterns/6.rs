// Using pattern &mut V to match a mutable reference needs you to be very careful, do to v being a
// value after matching.
//
// Given
//
// FIX the error with least changing
// DON'T remove any code line
/*fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       &mut value => value.push_str(" world!")
    }
}*/

// My Solution
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       value => value.push_str(" world!") // Remove &mut because r is already &mut reference to v
    }
}
