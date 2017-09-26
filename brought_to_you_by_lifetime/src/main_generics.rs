fn largest<T>(list: &[T]) -> &T
where T: std::cmp::PartialOrd {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// Rust performs monomorphization at compile time,
// for example:
/*
// This code:
let integer = Some(5);
let float = Some(5.0);
// will prompt the compiler to create the following enums:
*/
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("Let's learn about lifetimes!");

    // This is where the learning happens

    let v1 = vec![1, 2, 3, 4, -1, -2, 21, 0, -1];
    println!("Largest 1: {:?}", largest(&v1));
    let v2 = vec!["hello", "world", "make", "my", "morning"];
    println!("Largest 2: {:?}", largest(&v2));

    println!("Complete!");
}
