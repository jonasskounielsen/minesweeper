use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

use super::{PlaceI32, Cell, CellValue};

#[derive(Clone, Copy, Debug)]
pub struct CellBuilder {
    mine_concentration: f64,
    seed: u64,
}

impl CellBuilder {
    pub const DUMMY: CellBuilder = CellBuilder {
        mine_concentration: 0f64,
        seed: 0,
    };

    pub fn new(mine_concentration: f64, seed: Option<u64>) -> CellBuilder {
        CellBuilder {
            mine_concentration,
            seed: seed.unwrap_or_else(|| Self::get_random_seed()),
        }
    }

    pub fn cell(&self, place: PlaceI32) -> Cell {
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
        let cell = Cell::new(value);
        cell
    }

    fn get_random_seed() -> u64 {
        let mut rng = StdRng::from_os_rng();
        rng.next_u64()
        ;0xDEADBEEF
    }
}