// You can declare a new variable with the same name as a previous variable, here we can say **the
// first one is shadowed by the second one
//
// Given
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
/*fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x =  42;
    println!("{}", x); // Prints "42".
}*/

// My Solution
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // Changed value to twelve because x shadowed the previous declaration
    }

    assert_eq!(x, 5); // Changed valye to 5 because x is 5 in this scope. The 'showing x''s scope ended with the brace

    let x = 42;
    println!("{x}");
}
