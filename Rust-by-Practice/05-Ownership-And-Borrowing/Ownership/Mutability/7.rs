// Mutability can be changed when ownership is transfered
//
// Given
//
/*fn main() {
    let x = Box::new(5);
    
    let ...      // Implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}*/

// My Solution
//
// UNDONE
//
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(5);      // Declare y and make it mutable
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
