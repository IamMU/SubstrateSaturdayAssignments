// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for
// us
//
// Given
// 
// Modify `assert!` to make it work
/*fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");
}*/

// My Solution
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597); // Change to 1597

    println!("Success!");
}
