// Given
//
// Make it work
/*fn main() {
    let c1 = "中";
    print_char(c1);
}

fn print_char(c : char) {
    println!("{}", c);
}*/

// My Solution
fn main() {
    let c1 = '中'; // Change "" to '' to make it a char not a string literal. Very tricky problem x)
    print_char(c1);
}

fn print_char(c: char) {
    println!("{c}");
}
