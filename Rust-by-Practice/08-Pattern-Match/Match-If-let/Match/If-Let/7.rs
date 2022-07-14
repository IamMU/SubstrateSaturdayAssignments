// For some cases, when matching enums, match is too heavy, We can use if let instead.
//
// Given
//
// Fill in the blank
/*enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    __ {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}*/

// My Solution
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a { // Add if let statement for the Bar variant
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}
