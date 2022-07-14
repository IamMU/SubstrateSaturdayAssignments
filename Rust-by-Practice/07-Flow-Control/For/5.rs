// Given
//
/*fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.__ {
        println!("The {}th element is {}",i+1,v);
    }
}*/

// My solution
fn main() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i,v) in a.iter().enumerate() { // Add .iter() to make it iterateable and .enumerate to get index
        println!("The {}th element is {}",i+1,v);
    }
}

