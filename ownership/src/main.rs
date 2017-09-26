fn main() {
    println!("<program>");
    test();
    println!("</program>");
}

fn test() {
    // :: is to access a namespace method,
    // similar to clojure's `/` in `clojure.string/blank?`
    let mut s1 = String::from("Carlos");
    // s1 comes into scope, so `s1` is valid until it
    // "goes out of scope" (at which point it is invalid)
    println!("Hello, {}!", s1);

    s1.push_str(" Peña");
    let s2 = s1; // s1 reference is invalidated as it has "moved"

    println!("{}", s2);
    // println!("{}", s1); // Error "^^ value used here after move"

    // if we do not want our data to become invalidated,
    // we can use the method of String "clone"
    // This deeply copies the data structure and creates
    // a new reference.
    let s3 = s2.clone();
    println!("{} = {}", s2, s3);

    // Okay, so something interesting
    // References to variables are stored on the stack
    // And then, when those variables go out of scope,
    // their heap is freed.
    // If two variables point to the same heap space though,
    // (which can not happen in Rust), then there can result
    // a "double free" error which can lead to data corruption.

    // but what about this:
    let a = 1;
    let b = a;
    println!("a: {}, b: {}", a, b);
    // why is this valid?
    // well, for simple data types such as ints, bools, floats,
    // and tuples of only ints, bools, and floats, the data
    // is stored directly on the stack, because we can know
    // their size before allocating.
    let c = (1, 2, 3, 4.2, true, false);
    let d = c; // valid
    assert_eq!(c, d);

    let e = (1, 2, String::from("3"));
    let f = e.clone(); // clone required because of String
    assert_eq!(e, f);
    assert_eq!(e, f);

    println!("{}", first_word(&s2));
    println!("{}", first_word("Hello World"));
} // s2 is out of scope

// notice how &String implements &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
