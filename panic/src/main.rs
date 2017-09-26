use std::fs::File;
use std::io;
use std::io::Read;

struct hey {
    pub value: u32,
    s: u32,
}

fn calc(content: String) -> Result<f32, String> {
    println!("{:?}", content);
    Ok(0.0)
}

fn calc_file(file_name: &str) -> Result<f32, String> {
    fn open(file_name: &str) -> Result<String, io::Error> {
        let mut s = String::new();
        // question marks return Err
        File::open(file_name)?.read_to_string(&mut s)?;
        Ok(s)
    }
    match open(file_name) {
        Ok(s) => calc(s),
        Err(err) => Err(format!("Oh no! ({}) {:?}", file_name, err)),
    }
}

fn main() {
    println!("{:?}", calc_file("test.calc"));
    println!("{:?}", calc_file("test.calcs"));
    // panic! is used to signal that your code is in a state
    // that it is unable to handle.
    // Otherwise, a Result<_, _> should be used

    /*
    I really like the perscriptive use of Result generic
    and the generic Option. It reminds me of what the Java
    Optional tries to be, but isn't nearly as terse, as a
    result of the lacking match constructor.
    It is also really incredible how concise the typing is
    required for maintaining the Result and Err() / Ok()
    constructs. There is something magic about rust.
    */
}
