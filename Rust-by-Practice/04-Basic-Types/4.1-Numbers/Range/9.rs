// Two Goals:
// 1. Modify assert! to make it work
// 2. Make println! output: 97 - 122
// 
// Given
//
/*fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}*/

// My Solution
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5); // Change to -5 --- Goal 1 -> Done ---

    for c in 97..=122 { // Change range --- Goal 2 -> Done ---
        println!("{c}");
    }
}
