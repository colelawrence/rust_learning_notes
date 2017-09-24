extern crate communicator;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}
// self because:
// The only exception is in a use statement, where paths are relative to the crate root by default
use self::TrafficLight::{Red, Yellow};

fn main() {
    communicator::client::connect();
}