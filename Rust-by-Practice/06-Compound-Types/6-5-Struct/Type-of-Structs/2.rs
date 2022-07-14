// Unit struct don't have any fields. It can be useful when you need to implement a trait on some
// type but don't have nany data that you want to stor in the type itself.
//
// Given
//
/*struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}

// Fill the blank to make the code work
fn do_something_with_unit(u: __) {   }
*/

// My Solution
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
} 

fn do_something_with_unit(u: Unit) {   } // Take in Unit struct as parameter
