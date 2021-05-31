mod action;
mod cube;

use crate::action::{ACTION_B_INV, ACTION_D, ACTION_D2, ACTION_L, ACTION_L2, ACTION_B};
use crate::cube::{Cube, Facet, Facet::*};
use std::collections::{HashSet, VecDeque};

const TEST_DATA: [Facet; 24] = [
    RED, WHITE, ORANGE, WHITE, BLUE, RED, YELLOW, RED, BLUE, ORANGE, BLUE, BLUE, GREEN, GREEN,
    ORANGE, WHITE, YELLOW, WHITE, RED, GREEN, GREEN, ORANGE, YELLOW, YELLOW,
];

/// perform breadth-first search
fn bfs(query: &Cube) -> Option<Cube> {
    let root: Cube = Cube::solved();
    let mut seen: HashSet<[Facet; 24]> = HashSet::new();
    let mut q: VecDeque<Cube> = VecDeque::with_capacity(1e6 as usize);
    let facets = root.facets;
    q.push_back(root);
    seen.insert(facets);
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        if v.facets == query.facets {
            return Some(v);
        }
        for child in v.children() {
            if !seen.contains(&child.facets) {
                let facets = child.facets;
                q.push_back(child);
                seen.insert(facets);
            }
        }
    }
    return None;
}

fn main() {
    let mut query = Cube::new(TEST_DATA);

    // apply a bunch of random transformations, to make it harder...
    query = ACTION_D2.transform(&query);
    query = ACTION_B_INV.transform(&query);
    query = ACTION_L2.transform(&query);
    query = ACTION_D.transform(&query);
    query = ACTION_B_INV.transform(&query);
    query = ACTION_L2.transform(&query);
    query = ACTION_L.transform(&query);
    query = ACTION_B_INV.transform(&query);
    query = ACTION_D.transform(&query);
    query = ACTION_L.transform(&query);
    query = ACTION_B.transform(&query);

    if let Some(result) = bfs(&query) {
        println!("{:?}", result.history)
    }
}
