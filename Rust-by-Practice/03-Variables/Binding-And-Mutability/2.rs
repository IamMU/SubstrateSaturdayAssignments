// Use mut to mark variable as mutable.
//
// Given
//
// // Fill the blanks in the code to make it compile
/*fn main() {
    let __ =  1;
    __ += 2;

    assert_eq!(x, 3);
    println!("Success!");
}*/

// My Solution

fn main() {
    let mut x = 1; // Change variable name from __ to x so asset_eq! does not panic. Add mut keyword to make it mutable
    x += 2; 

    assert_eq!(x, 3);

    println!("Success!")
}
