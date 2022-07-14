// Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side
// of an assignment
//
// Given
//
/*fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
} */

// My Solution
fn main() {
    let (x, y);

    (x, ..) = (3, 4); // X is 3
    [.., y] = [1, 2]; // Y is 2

    assert_eq!([x, y], [3, 2]); 

    println!("Success!");
}
