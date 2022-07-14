// Given
//
/*fn main() {
   let v = (let x = 3);

   assert!(v == 3);

   println!("Success!");
}*/

// My Solution
fn main() {
    let v = {
        let x = 3;
        x // Add x as return value
    };

    assert!(v == 3);

    println!("Success!");
}

