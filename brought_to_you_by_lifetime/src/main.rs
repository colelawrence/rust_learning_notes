mod measurable;

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
fn longestm<'a, M>(a: &'a M, b: &'a M) -> &'a M
where M: measurable::Measurable {
    if a.measured_length() > b.measured_length() { a } else { b }
}

use std::collections::HashMap;

type Edge<'a, T> = (&'a T, f32);
struct G <'a, T: 'a> {
    vertices: Vec<T>,
    edges: HashMap<&'a T, Vec<Edge<'a, T>>>,
}

/*
// Check out these lifetimes!
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/
impl measurable::Measurable for MKind {
    fn measured_length(&self) -> f64 {
        match self {
            MKind::f64(v) => v.measured_length(),
            MKind::String(v) => v.measured_length(),
        }
    }
}
#[derive(Debug)]
enum MKind<'a> {
    String(&'a String),
    f64(&'a f64),
}

fn main() {
    let s1 = String::from("abcdef");
    let s2 = String::from("xyz");

    let result = longest(&s1, &s2);
    println!("longest be {}!", result);

    let m1 = 2.8;
    let m2 = 2.8;
    let result2 = longestm(&m1, &m2);
    println!("longest be {}!", result2);

}