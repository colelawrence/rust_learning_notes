struct Sandwich {
    Bread: i32,
}
struct Burrito {
    Bread: i32,
}

enum Food {
    Sandwich(Sandwich),
    Burrito(Burrito),
}

use std::fmt::Formatter;
use std::fmt;

impl std::fmt::Display for Sandwich {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sandwich {}", self.Bread)
    }
}

fn main() {
    let instructions = ["cook", "eat", "sleep"];

    let mut index = 0;
    // scopes are pretty neat.
    // the same concept is shared throughout
    // the language syntax; that \{ starts a scope
    // and \} ends a scope.
    for instruction in instructions.iter() {
        index = index + 1;
        println!("{}. {}", index, instruction);
    }

    let sand = Sandwich { Bread: 1, };
    println!("{}", sand);

    for num in (1..5) {
        println!("hey {}", num);
    }

    println!("End");
}
