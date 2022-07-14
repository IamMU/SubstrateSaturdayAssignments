// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for
// us.
//
// Given
//
//  Fill the blank
/*fn main() {
    let v: u16 = 38_u8 as __;

    println!("Success!");
}*/

// (NOT) My Solution
fn main() {
    let v: u16 = 38_u8 as u16; // I learned it rn that we could do this

    println!("Success!");
}
