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

    for num in (1..5) {
        println!("hey {}", num);
    }

    println!("End");
}
