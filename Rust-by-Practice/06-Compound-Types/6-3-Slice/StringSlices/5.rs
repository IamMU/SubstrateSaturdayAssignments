// Given
//
/*fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..2];

    assert!(slice == "你");

    println!("Success!");
}*/

// My Solution
fn main() {
    let s = "你好，世界";
    let slice = &s[..3]; // Change range to ..3 because these characters take 3 bytes

    assert!(slice == "你");

    println!("Success!");
}
