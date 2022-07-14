// Given
//
/*fn main() {
   let x = 5;
   // Fill the blank
   let p = __;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}*/

// My Solution
fn main() {
    let x = 5;

    let p = &x; // Make a reference to the mem address of x

    println!("the memory address of x is {:p}", p);
}
