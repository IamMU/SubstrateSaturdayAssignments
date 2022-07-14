// A scope is the range within the program for which the item is valid
//
// Given
//
// Fix the error below with least amount of modification
/*fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}*/

// My Solution

fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {x} and value of y is {y}");
    }
    println!("The value of x is {x}"); // Remove y because it is no longer a valid variable
}
