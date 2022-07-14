// &String can be implicitly converted into &str
//
// Given
//
// Fix errors
/*fn main() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_word` need a `&str` type.
    // It works because `&String` can be implicitly converted to `&str, If you want know more ,this is called `Deref`
    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &str) -> &str {
    &s[..1]
}*/

// My Solution
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word); // Move println! here so it doesn't give error as we clear the string, where word is referencing to
    s.clear();
}
fn first_word(s: &str) -> &str {
    &s[..1]
}
