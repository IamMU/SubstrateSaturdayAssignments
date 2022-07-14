// What's the size of the unit type?
//
// Given
//
// Modify `4` in assert to make it work
use std::mem::size_of_val;
/*fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 4);

    println!("Success!");
}*/

// My Solution
fn main() {
    let unit: () = ();

    assert!(size_of_val(&unit) == 0); // Change 4 to 0 

    println!("Success!");
}
