// Given
//
// Make println! work
/*fn main() {
    let _f: bool = false;

    let t = true;
    if !t {
        println!("Success!");
    }
}*/

// My Solution
fn main() {
    let _f: bool = false;

    let t = true;

    if t { // Remove ! to make it true
        println!("Success!");
    }
}

