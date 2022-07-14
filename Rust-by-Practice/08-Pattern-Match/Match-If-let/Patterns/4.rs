// A match guard is an additional if condition specified after the pattern in a match arm that must
// also match, along with pattern matching, for that arm to be chosen.
//
// Given
//
// Fill in the blank to make the code work, `split` MUST be used
/*fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) __ => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}*/

// My Solution
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split), // Add guard to check if x is less than split
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

