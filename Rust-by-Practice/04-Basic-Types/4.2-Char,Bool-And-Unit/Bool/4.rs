// Given
//
// Make it work
/*fn main() {
    let f = true;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}*/

// My Solution
fn main() {
    let f = true;
    let t = true && false;

    assert_eq!(t, !f); // Add ! before f to make it false and hence equal to t

    println!("Success!");
}
