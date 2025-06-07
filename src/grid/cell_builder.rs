use rand::{Rng, rngs::StdRng, SeedableRng};

use super::{Place, Cell, CellValue};

#[derive(Clone, Copy, Debug)]
pub struct CellBuilder {
    mine_concentration: f32,
    seed: u64,
}

impl CellBuilder {
    pub const DUMMY: CellBuilder = CellBuilder {
        mine_concentration: 0f32,
        seed: 0,
    };

    pub fn new(mine_concentration: f32, seed: u64) -> CellBuilder {
        CellBuilder {
            mine_concentration,
            seed,
        }
    }

    pub fn cell(&self, place: Place) -> Cell {
        let mut seed = [42; 32];
        seed[ 0..8 ].copy_from_slice(&self.seed.to_be_bytes());
        seed[ 8..12].copy_from_slice(&place.x  .to_be_bytes());
        seed[12..16].copy_from_slice(&place.y  .to_be_bytes());

        let mut rng = StdRng::from_seed(seed);

        let value = if self.mine_concentration > rng.random() {
            CellValue::Mine
        } else {
            CellValue::Empty
        };
        let value = if place.y < 0 { CellValue::Mine } else { CellValue::Empty }; // dbg
        let mut cell = Cell::new(value);
        cell.reveal();
        cell
    }
}