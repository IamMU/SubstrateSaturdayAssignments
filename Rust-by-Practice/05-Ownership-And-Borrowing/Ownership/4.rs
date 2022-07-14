// Given
//
// Fix the error without removing code line
/*fn main() {
    let s = String::from("hello, world");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}*/

// My Solution
fn main() {
    let s = String::from("hello, world");

    print_str(&s); // Use a refernce instead of var

    println!("{}", s);
}

fn print_str(s: &String)  { // Change return type to string refernce
    println!("{}",s)
}
