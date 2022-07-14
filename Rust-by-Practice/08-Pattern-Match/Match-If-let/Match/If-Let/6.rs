// For some cases, when matching enums, match is too heavy. We can use if let instead.
//
// Given
//
/*fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
        _ => {}
    };
}*/

// My Solution
fn main() {
    let o = Some(7);

    // Add if let
    if let Some(i) = o {
        println!("This is a very long string and {:?}", i);
    }
}

