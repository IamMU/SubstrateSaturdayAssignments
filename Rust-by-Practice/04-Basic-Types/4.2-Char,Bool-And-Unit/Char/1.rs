// Given 
//
// Make it work
use std::mem::size_of_val;
/*fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!");
}*/

// My Solution
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // One char takes 4 bytes, whether normal or unicode

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); // Change to 4 because unicode characters in Rust take 4 bytes

    println!("Success!");
}
