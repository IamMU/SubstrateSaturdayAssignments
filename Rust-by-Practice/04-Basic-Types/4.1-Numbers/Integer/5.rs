// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for
// us.
//
// Given
//
// Fix errors and panics to make it work
/*fn main() {
   let v1 = 251_u8 + 8;
   let v2 = i8::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}*/

// My Solution
fn main() {
    let v1 = 247_u8 + 8; // Change 251 to 247 so that it does not make an overflow error
    let v2 = u8::checked_add(247, 8).unwrap(); // Change i8 to u8 and change 251 to 247 so it doesn't panic
    println!("{v1}, {v2}");
}

