// Given
//
/*fn main() {
    // Fill the blank with proper array type
    let arr: __ = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 4);

    println!("Success!");
}*/

// My Solution
fn main() {
    let arr: [u32; 5] = [1, 2, 3, 4, 5]; // Specify data type and size of array

    assert!(arr.len() == 5); // Change to 5

    println!("Success!");
}
