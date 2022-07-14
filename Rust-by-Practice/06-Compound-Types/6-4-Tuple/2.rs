// Members ca be extracted from tuple using indexing
//
// Given
//
// Make it work
/*fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "sunface");

    println!("Success!");
}*/

// My Projects
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface"); // Change to 2 because sunface is at index 2

    println!("Success!");
}
