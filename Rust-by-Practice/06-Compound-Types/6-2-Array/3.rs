// All elements in an array can be initialized to the same value at once.
//
// Given
//
/*fn main() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    assert!(std::mem::size_of_val(&arr) == 12); // 12 because 3 chars in array

    println!("Success!");
}*/

// My Solution
fn main() {
    let list: [i32; 100] = [1; 100] ; // Add 1 to array 100 times

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
