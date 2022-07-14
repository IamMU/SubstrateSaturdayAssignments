// Given
//
/*fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, y);

    println!("Success!");
}*/

// My Solution
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y); // Dereference y to get the value

    println!("Success!")
}
