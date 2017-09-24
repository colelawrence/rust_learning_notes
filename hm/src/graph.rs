
use std::collections::HashMap;
use std;

type Edge = (usize, f32);
#[derive(Debug)]
pub struct G<T> {
    vertices: Vec<T>,
    edges: HashMap<usize, Vec<Edge>>,
}

type Walk = (Vec<usize>, f32);
fn new_walk() -> Walk {
    (Vec::new(), 0.0)
}
fn walk(from: &Walk, to: usize, distance: f32) -> Walk {
    let (mut prev, mut dist) = from.clone();
    prev.push(to);
    dist += distance;
    (prev, dist)
}
#[derive(Debug)]
struct PathFinder {
    seen: HashMap<usize, Walk>,
    next: HashMap<usize, Walk>,
}
impl<T> G<T> {
    pub fn shortest_path(&self, from: usize, to: usize) -> Option<(Vec<&T>, f32)>
    where
        T: std::fmt::Debug,
    {
        let mut path = PathFinder::new();
        let mut solution: Option<(Vec<&T>, f32)> = None;
        path.next.insert(from, (vec![from], 0.0));
        'outer: loop {
            let opts = path.next;
            println!("Next up: {:?}", opts);
            // TODO: sort options by total distance,
            // ensure that the longest paths have been walked
            // let options: Vec<(f32, usize)> = Vec::new();
            let mut next = HashMap::new();
            for (&choice_id, &(ref history, distance)) in opts.iter() {
                if choice_id == to {
                    println!("{:?}", self);
                    println!("Found something... {:?}", history);
                    let history_t = history
                        .iter()
                        .map(|&id| self.get_vertice(id).unwrap())
                        .collect();
                    solution = Some((history_t, distance));
                    break 'outer;
                }
                path.seen.insert(choice_id, (history.clone(), distance));
                if let Some(next_to) = self.to(choice_id) {
                    for &(next_id, next_id_distance) in next_to {
                        if !path.seen.contains_key(&next_id) {
                            let mut next_history = history.clone();
                            next_history.push(next_id);
                            let next_walk: Walk = (next_history, next_id_distance + distance);
                            next.insert(next_id, next_walk);
                        }
                    }
                }
            }
            if next.is_empty() {
                break;
            }
            path.next = next;
        }
        solution
    }
}
impl PathFinder {
    fn new() -> PathFinder {
        PathFinder {
            seen: HashMap::new(),
            next: HashMap::new(),
        }
    }
}

impl<T> G<T> {
    pub fn add_vertice(&mut self, v: T) -> usize {
        self.vertices.push(v);
        self.vertices.len() - 1
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f32) {
        let es = self.edges.entry(from).or_insert(Vec::new());
        es.push((to, weight));
    }

    pub fn to(&self, id: usize) -> Option<&Vec<(usize, f32)>> {
        match self.edges.get(&id) {
            Some(edges) => Some(edges),
            None => None,
        }
    }

    pub fn get_vertice(&self, id: usize) -> Option<&T> {
        self.vertices.get(id)
    }

    pub fn new() -> G<T> {
        G {
            vertices: Vec::new(),
            edges: HashMap::new(),
        }
    }
}
