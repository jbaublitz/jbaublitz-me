use std::convert::TryFrom;
use std::marker::PhantomData;
use std::slice::Iter;

#[derive(Copy,Clone)]
pub enum Index {
    One,
    Two,
    Three,
    Four
}

impl<'a> Into<usize> for &'a Index {
    fn into(self) -> usize {
        *self as usize
    }
}

impl TryFrom<usize> for Index {
    type Error = super::JbaublitzError;

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        let idx = match v {
            0 => Index::One,
            1 => Index::Two,
            2 => Index::Three,
            3 => Index::Four,
            _ => return Err(super::JbaublitzError("Failed to convert to type Index")),
        };
        Ok(idx)
    }
}

pub struct Position<'a> {
    vert: &'a Index,
    horiz: &'a Index,
}

impl<'a> Into<usize> for Position<'a> {
    fn into(self) -> usize {
        let (vert, horiz): (usize, usize) = (self.vert.into(), self.horiz.into());
        vert * 4 + horiz
    }
}

pub struct Mat4([f32; 16]);

impl Mat4 {
    const INDEX_LIST: [Index; 4] = [Index::One, Index::Two, Index::Three, Index::Four];

    pub fn new() -> Self {
        let mut arr = Mat4([0.0; 16]);
        arr.set(&Index::One, &Index::One, 1.0);
        arr.set(&Index::Two, &Index::Two, 1.0);
        arr.set(&Index::Three, &Index::Three, 1.0);
        arr.set(&Index::Four, &Index::Four, 1.0);
        arr
    }

    fn index_iter<'a>() -> Iter<'a, Index> {
        [Index::One, Index::Two, Index::Three, Index::Four].into_iter()
    }

    pub fn as_f32_slice(&self) -> &[f32] {
        &self.0
    }

    pub fn get_position(&self, pos: Position) -> f32 {
        let vert_usize: usize = pos.vert.into();
        let horiz_usize: usize = pos.horiz.into();
        self.0[vert_usize * 4 + horiz_usize]
    }

    pub fn get(&self, vert: &Index, horiz: &Index) -> f32 {
        self.get_position(Position { vert, horiz, })
    }

    pub fn set_position(&mut self, pos: Position, val: f32) {
        let vert_usize: usize = pos.vert.into();
        let horiz_usize: usize = pos.horiz.into();
        self.0[vert_usize * 4 + horiz_usize] = val;
    }

    pub fn set(&mut self, vert: &Index, horiz: &Index, val: f32) {
        self.set_position(Position { vert, horiz, }, val)
    }

    fn calc_cell(&self, chain: &Mat4, comp_vert: &Index, comp_horiz: &Index) -> f32 {
        Self::index_iter().fold(0.0, |acc, idx| {
            acc + self.get(comp_vert, idx) * chain.get(idx, comp_horiz)
        })
    }

    pub fn chain(&mut self, chain: Mat4) {
        for idx_vert in Self::index_iter() {
            // Allocate one spare row
            let mut spare = [0f32; 4];
            for idx_horiz in Self::index_iter() {
                let idx_horiz_usize: usize = idx_horiz.into();
                spare[idx_horiz_usize] = self.calc_cell(&chain, idx_vert, idx_horiz);
            }
            for (idx, val) in spare.into_iter().enumerate() {
                self.set(idx_vert, &Index::try_from(idx).unwrap(), *val);
            }
        }
    }

    pub fn translate(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        let mut translation = Mat4::new();
        translation.set(&Index::One, &Index::Four, x.unwrap_or(0.0));
        translation.set(&Index::Two, &Index::Four, y.unwrap_or(0.0));
        translation.set(&Index::Three, &Index::Four, z.unwrap_or(0.0));
        self.chain(translation);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain() {

    }
}
