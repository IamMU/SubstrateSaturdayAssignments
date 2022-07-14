// Fill the blanks
use std::ops::{Range, RangeInclusive};
/*fn main() {
    assert_eq!((1..__), Range{ start: 1, end: 5 });
    assert_eq!((1..__), RangeInclusive::new(1, 5));

    println!("Success!");
}*/

// My Solution
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 }); // 5 to not include 5
    assert_eq!((1..=5_), RangeInclusive::new(1, 5)); // =5 to include 5

    println!("Success!");
}
