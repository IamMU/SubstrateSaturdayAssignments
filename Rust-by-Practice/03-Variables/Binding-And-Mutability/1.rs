// 1. A variable can be used only if it has been initialized
//
// Given Code
//
// Fix the error below with least amount of modification to the code
/*fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}*/

// My Solution
    
fn main() {
    let x: i32 = 5; // Change value to 5, so asset_eq does not panic
    let y: i32;

    assert_eq!(x, 5);
    println!("Success")
}

