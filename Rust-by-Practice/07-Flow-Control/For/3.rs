// The for in construct can be used to iterate through an iterator, e. a range a..b.
//
// Given
//
/*fn main() {
    for n in 1..=100 { // modify this line to make the code work
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
}*/

// My Solution
fn main() {
    for n in 1..100 { // Remove =
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }

    println!("Success!");
} 
