// We must specify concrete value for each of the fields in struct.
//
// Given
//
// Fix the error
/*struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
    };

    println!("Success!");
} */

// My Solution
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age: 24,
        hobby: String::from("procrastinating"),  // Add missing values
    };

    println!("Success!");
} 
