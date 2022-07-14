// Given
//
/*fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".__ {
        println!("{}", c)
    }
}*/

// My Solution
fn main() {
    for c in "你好，世界".chars() { // Add .chars() to get an chars iterateble
        println!("{}", c)
    }
}

