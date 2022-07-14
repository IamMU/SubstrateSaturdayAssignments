// Given
//
/*fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: __ = __;
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}*/

// My Solution
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..=3]; // Add datatype and make range of elements with index 1 to 3
    assert_eq!(slice, &[2, 3, 4]); 

    println!("Success!");
}

