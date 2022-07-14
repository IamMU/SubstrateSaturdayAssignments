// matches! looks like match, but can do something different.
//
// Given
//
/*fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(__)
    }

    println!("Success!");
}*/

// My Solution
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
         assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9')) // Add matches! expression here with char ranges and number range
    }

    println!("Success!");
} 

