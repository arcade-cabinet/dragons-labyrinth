#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Axial { pub q: i32, pub r: i32 }

pub const DIRS: [(i32,i32);6] = [(0,-1),(1,-1),(1,0),(0,1),(-1,1),(-1,0)];

impl Axial {
    pub fn neighbors(self) -> [Axial;6] {
        let mut out = [self;6];
        for (i, (dq, dr)) in DIRS.iter().enumerate() {
            out[i] = Axial{ q: self.q + dq, r: self.r + dr };
        }
        out
    }
}

