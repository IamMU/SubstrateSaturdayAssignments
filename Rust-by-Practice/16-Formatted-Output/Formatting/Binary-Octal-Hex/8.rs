// Given

// fn main() {
//     assert_eq!(format!("__", 27), "0b11011");
//     assert_eq!(format!("__", 27), "0o33");
//     assert_eq!(format!("__", 27), "0x1b");
//     assert_eq!(format!("__", 27), "0x1B");

//     println!("{:x}!", 27); // hex with no prefix => 1b

//     println!("{:#010b}", 27); // pad binary with 0, width = 10,  => 0b00011011

//     println!("Success!")
// }

// My Solution
fn main() {
    assert_eq!(format!("{:#b}", 27), "0b11011");
    assert_eq!(format!("{:#o}", 27), "0o33");
    assert_eq!(format!("{:#x}", 27), "0x1b");
    assert_eq!(format!("{:#X}", 27), "0x1B");

    println!("{:x}!", 27); // hex with no prefix => 1b

    println!("{:#010b}", 27); // pad binary with 0, width = 10,  => 0b00011011

    println!("Success!")
}
