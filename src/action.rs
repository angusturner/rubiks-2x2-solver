use crate::action::ActionKind::*;
use crate::cube::Cube;

// define all the actions
pub const ACTION_L: Action = Action {
    kind: L,
    source: [0, 2, 4, 6, 21, 20, 15, 13, 16, 17, 19, 18],
    target: [4, 6, 21, 20, 15, 13, 0, 2, 17, 19, 18, 16],
};
pub const ACTION_L_INV: Action = ACTION_L.invert();

pub const ACTION_L2: Action = Action {
    kind: L2,
    source: [0, 2, 4, 6, 21, 20, 15, 13, 16, 17, 19, 18],
    target: [21, 20, 15, 13, 0, 2, 4, 6, 19, 18, 16, 17],
};
pub const ACTION_B: Action = Action {
    kind: B,
    source: [9, 11, 22, 20, 18, 16, 0, 1, 12, 14, 15, 13],
    target: [0, 1, 9, 11, 22, 20, 18, 16, 13, 12, 14, 15],
};
pub const ACTION_B_INV: Action = ACTION_B.invert();
pub const ACTION_B2: Action = Action {
    kind: B2,
    source: [22, 20, 18, 16, 0, 1, 9, 11, 14, 15, 13, 12],
    target: [0, 1, 9, 11, 22, 20, 18, 16, 13, 12, 14, 15],
};
pub const ACTION_D: Action = Action {
    kind: D,
    source: [6, 7, 10, 11, 14, 15, 18, 19, 20, 21, 23, 22],
    target: [10, 11, 14, 15, 18, 19, 6, 7, 21, 23, 22, 20],
};
pub const ACTION_D_INV: Action = ACTION_D.invert();
pub const ACTION_D2: Action = Action {
    kind: D2,
    source: [6, 7, 10, 11, 14, 15, 18, 19, 20, 21, 23, 22],
    target: [14, 15, 18, 19, 6, 7, 10, 11, 23, 22, 20, 21],
};

pub const ACTIONS: [Action; 9] = [
    ACTION_L,
    ACTION_L_INV,
    ACTION_L2,
    ACTION_B,
    ACTION_B_INV,
    ACTION_B2,
    ACTION_D,
    ACTION_D_INV,
    ACTION_D2,
];

#[derive(Debug, Copy, Clone)]
pub enum ActionKind {
    L,
    LInv,
    L2,
    B,
    BInv,
    B2,
    D,
    DInv,
    D2,
}

#[derive(Debug, Copy, Clone)]
pub struct Action {
    pub(crate) kind: ActionKind,
    pub(crate) source: [usize; 12],
    pub(crate) target: [usize; 12],
}

impl Action {
    /// generate an inverse action
    pub(crate) const fn invert(&self) -> Self {
        let kind = match self.kind {
            L => LInv,
            LInv => L,
            B => BInv,
            BInv => B,
            D => DInv,
            DInv => D,
            _ => self.kind,
        };
        Self {
            kind,
            source: self.target,
            target: self.source,
        }
    }

    /// apply the transformation to a cube, to yield a new cube
    pub(crate) fn transform(&self, cube: &Cube) -> Cube {
        // transform the facets
        let mut new_facets = cube.facets;
        for (trg_idx, src_idx) in self.target.iter().zip(&self.source) {
            new_facets[*trg_idx] = cube.facets[*src_idx];
        }

        // update the history
        let mut new_history = cube.history.clone();
        new_history.push(self.kind);

        Cube {
            facets: new_facets,
            history: new_history,
        }
    }
}
