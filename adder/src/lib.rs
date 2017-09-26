#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn panic_if_lt_1(num: i32) {
    if num < 1 {
        panic!("it's less than 1! {}", num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const larger: Rectangle = Rectangle { length: 20, width: 10 };
    const smaller: Rectangle = Rectangle { length: 19, width: 9 };
    const smaller2: Rectangle = Rectangle { length: 10, width: 5 };
    const not_smaller: Rectangle = Rectangle { length: 21, width: 5 };
    const not_smaller2: Rectangle = Rectangle { length: 19, width: 11 };

    #[test]
    fn it_larger_can_hold_smaller() {
        assert!(larger.can_hold(&smaller), "Larger must hold smaller: {:?}", larger);
        assert!(larger.can_hold(&smaller2));
    }

    #[test]
    fn it_smaller_cannot_hold_larger() {
        assert!(!larger.can_hold(&not_smaller));
        assert!(!larger.can_hold(&not_smaller2));
    }

    #[test]
    #[should_panic(expected = "it's less than 1")]
    fn it_panics() {
        panic_if_lt_1(-1);
    }
    #[test]
    fn it_dn_panic() {
        panic_if_lt_1(1);
        panic_if_lt_1(2);
    }
}
// Continue reading about testing: https://doc.rust-lang.org/book/second-edition/ch11-02-running-tests.html
