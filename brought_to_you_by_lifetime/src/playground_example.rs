mod measurable {
    pub trait Measurable {
        fn measured_length(&self) -> f64;
    }
    impl Measurable for String {
        fn measured_length(&self) -> f64 {
            self.len() as f64
        }
    }
    impl Measurable for f64 {
        fn measured_length(&self) -> f64 {
            self.clone() as f64
        }
    }
}
fn longestm<'a, M>(a: &'a M, b: &'a M) -> &'a M
where
    M: measurable::Measurable,
{
    if a.measured_length() > b.measured_length() {
        a
    } else {
        b
    }
}
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

    let result = longestm(&s1, &s2);
    println!("longest be {}!", result);

    let f1 = 2.8;
    let f2 = 2.8;
    let result2 = longestm(&f1, &f2);
    println!("longest be {}!", result2);

    // Goal: compare unlike things
    let m1 = MKind::f64(&f1);
    let m2 = MKind::String(&s2);
    let result3 = longestm(&m1, &m2);
    println!("Dissimilar longest be: {:?}!", result3);
}
