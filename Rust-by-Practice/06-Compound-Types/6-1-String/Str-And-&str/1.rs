// We can't use str type in normal ways, but we can use &str.
//
// Given
//
// Fix error without adding new line
/*fn main() {
    let s: str = "hello, world";

    println!("Success!");
}*/

// My Solution
fn main() {
    let s: &str = "hello, world"; // Change data type to &str

    println!("Success!");
}

