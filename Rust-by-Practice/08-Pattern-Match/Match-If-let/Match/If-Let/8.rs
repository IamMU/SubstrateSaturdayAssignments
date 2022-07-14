// For some cases, when matching enums, match is too heavy. We can use if let instead.
//
// Given
//
/*enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    // Remove the codes below, using `match` instead
    if let Foo::Bar = a {
        println!("match foo::bar")
    } else if let Foo::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }
}*/

// My Solution

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    // Convert to match 
    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo:baz"),
        _ => println!("match others")
    }
}
