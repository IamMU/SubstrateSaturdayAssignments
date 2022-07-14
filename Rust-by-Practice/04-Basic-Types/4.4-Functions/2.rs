// Given
/*fn main() {
   print();
}

// Replace i32 with another type
fn print() -> i32 {
   println!("Success!");
}*/

// My Solution
fn main() {
   print();
}

// Replace i32 with another type
fn print() -> () { // Replace i32 with unit type, (), because the function returns nothing
   println!("Success!");
}
