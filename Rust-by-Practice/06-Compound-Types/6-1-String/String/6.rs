// You can only concat a String with &str , and String's ownership can be moved to another variable
//
// Given
//
// Fix errors without removing any line
/*fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}*/

// My Solution
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; // Clone s1 so we can use it in the println! macro and we s2 a refrence
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}

