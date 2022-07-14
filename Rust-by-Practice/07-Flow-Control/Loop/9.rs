// Loop is usually used togetther with break or continue
//
// Given
//
// Fill in the blanks
/*fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            __;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            __;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}*/

// My Solution

// Fill in the blanks
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue; // contnue statement
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break; // Break loop
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}

