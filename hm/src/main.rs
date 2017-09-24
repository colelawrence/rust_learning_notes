
use std::collections::HashMap;

pub mod graph;

use graph::G;

fn main() {

    let mut hm = HashMap::new();

    let some_words = "hello hello how are you my friend friend";

    for word in some_words.split_whitespace() {
        let count = hm.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", hm);

    let mut g: G<&str> = G::new();

    let cole = g.add_vertice("Cole");
    let carlos = g.add_vertice("Carlos");
    let george = g.add_vertice("George");
    let abbie = g.add_vertice("Abbie");

    g.add_edge(cole, carlos, 1.0);
    g.add_edge(carlos, cole, 1.0);
    g.add_edge(carlos, george, 1.0);
    g.add_edge(george, abbie, 1.0);
    g.add_edge(george, carlos, 2.0);
    g.add_edge(george, cole, 4.0);
    g.add_edge(george, carlos, 1.0);

    println!("{:?}", g);

    println!("George to Cole: {:?}", g.shortest_path(george, cole));
    println!("Cole to Abbie: {:?}", g.shortest_path(cole, abbie));
    println!("Cole to George: {:?}", g.shortest_path(cole, george));
    println!("Abbie to Carlos: {:?}", g.shortest_path(abbie, carlos));
    println!("Complete!");
}
