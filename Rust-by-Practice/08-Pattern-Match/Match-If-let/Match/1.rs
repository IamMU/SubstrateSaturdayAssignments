// Given
//
// Fill the blanks
/*enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        __ => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"),
    };
}*/

// My Solution

// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // Add South and North variants with the or operator
            println!("South or North");
        },
        _ => println!("West"),
    };
}

