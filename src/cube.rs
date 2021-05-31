use crate::action::{ActionKind, ACTIONS};
use crate::cube::Facet::*;
use std::slice::Iter;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Facet {
    WHITE,
    RED,
    BLUE,
    ORANGE,
    GREEN,
    YELLOW,
}

impl Facet {
    pub fn iterator() -> Iter<'static, Facet> {
        static FACET: [Facet; 6] = [WHITE, RED, BLUE, ORANGE, GREEN, YELLOW];
        FACET.iter()
    }
}

#[derive(Debug)]
pub struct Cube {
    pub(crate) facets: [Facet; 24],
    pub(crate) history: Vec<ActionKind>,
}

impl Cube {
    /// Create a new cube instance
    pub(crate) fn new(facets: [Facet; 24]) -> Self {
        Self {
            facets,
            history: vec![],
        }
    }

    /// Returns a solved instance of the `Cube` struct
    pub(crate) fn solved() -> Self {
        let mut facets = [WHITE; 24];
        let mut n: usize = 0;
        for elem in Facet::iterator() {
            for _ in 0..4 {
                facets[n] = *elem;
                n += 1;
            }
        }

        Self::new(facets)
    }

    pub(crate) fn children(&self) -> Vec<Cube> {
        ACTIONS.iter().map(|x| x.transform(self)).collect()
    }
}
