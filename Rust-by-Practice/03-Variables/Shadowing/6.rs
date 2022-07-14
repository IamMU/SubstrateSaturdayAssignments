// You can declare a new variable with the same name as a previous variable, here we can say **the
// first one is shadowed by the second one.
//
// Given
//
// Remove a line in the code to make it compile
/*fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}*/

// My Solution
fn main() {
    let mut x: i32 = 1;
    x = 7;
    
    let x = x;
    // x += 3; Removed this line because when we shadowed x, we didn't set it to be mutable, making
    // it immutable

    let y = 4;

    let y = "I can also be bound to text!:";

    println!("Success");
}
