// A Vec can be extended with extend method

// Given

// FILL in the blank
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
    
//     let mut v2 = Vec::new();
//     v2.__;

//     assert_eq!(v1, v2);

//     println!("Success!")
// }

// My Solution
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    
    let mut v2 = Vec::new();
    v2.extend(&v1); // Add elements of v1 to v2

    assert_eq!(v1, v2);

    println!("Success!")
}