
// Return search string and filename
fn get_args() -> (String, String) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        panic!("Need three arguments! minigrep {search string} {file name}");
    }
    (args[1].clone(), args[2].clone())
}

fn main() {
    let (search, file) = get_args();
    println!("Search for {search:?} in {file}", search=search, file=file);
}
