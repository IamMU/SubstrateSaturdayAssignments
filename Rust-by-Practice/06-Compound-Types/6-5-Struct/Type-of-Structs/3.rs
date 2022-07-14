// Tuple struct looks similar to typles, it has added meaning the struct name provides but has no
// named fields. It's useful when you want to give the whole tuple, but don't care about the
// fields's names.
//
// Given
//
// Fix the error and fill the blanks
/*struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Point(__, __, __);
    check_color(v);

    println!("Success!");
}

fn check_color(p: Color) {
    let (x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(__, 255);
 }*/

// My Solution
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Color(0, 127, 255); // Change struct name to Color :) as we are checking color, also add values that do not conflict with assert!
    
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Color) {
    let Color(x, _, _) = p; // Add Color struct 
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }
