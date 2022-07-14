// Given
//
// Don't use clone ,use copy instead
/*fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}*/

// My Solution
fn main() {
    let x = (1, 2, (), "hello"); // Removed to_string
    let y = x; // Simply assign x to y
    println!("{:?}, {:?}", x, y);
}

