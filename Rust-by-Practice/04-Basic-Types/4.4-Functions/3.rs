// Given
//
// Solve it in two ways
// DON'T let `println!` works
/*fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    
}*/

// My Solution

// Way One
// Use loop to never allow the code to proceed
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    loop {
        println!("Not gonna let you get through >:D");
    }
}

// Way Two
// I can't think of any 
