// Fix the warning below with:
//
// - {one star} Only one solution
// - {two star} Two distinct solutions
//
// Given
/*fn main() {
    let x = 1; 
}*/

// Warning: unused variable: `x`
//
// My Solution #1
//
// Tell rust that we know it is an unused variable
/*fn main() {
    let _x = 1; // Add _ before the name of the variable
}*/
// (Not) My Solution #2 - I didn't know we could do this :D
// Tell Rust not to give us unused variable warnings
#[allow(unused_variables)]
fn main() {
    let x = 1;
}
