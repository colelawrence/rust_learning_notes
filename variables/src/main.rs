macro_rules! trace {
    ($e:expr) => {
        println!("TRACE ({}:{}) => {:?}", file!(), line!(), $e);
    }
}

fn greet(name: &str) {
    println!("Hello, {}!", name)
}

fn len(name: &str) -> i64 {
    // notice importance of no semicolon here:
    // otherwise the type would be ()
    3
}

fn main() {
    greet("Carlos");
    // let statement : identifier assigned to expression
    let guess: usize = "42".parse().expect("number");
    // semicolons denote the end of a statement
    let x = 1;
    let y = {
        let x = x + 2;
        x + 1
    };
    /*
        hello
    */
    trace!(x);
    trace!(y);
    trace!(len("hey"));
}
