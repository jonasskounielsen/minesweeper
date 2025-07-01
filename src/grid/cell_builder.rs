use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

use super::{PlaceI32, Cell, CellValue};

#[derive(Clone, Copy, Debug)]
pub struct CellBuilder {
    mine_concentration: f64,
    seed: u64,
    origin: PlaceI32,
}

impl CellBuilder {
    pub const DUMMY: CellBuilder = CellBuilder {
        mine_concentration: 0f64,
        seed: 0,
        origin: PlaceI32 {
            x: 0,
            y: 0,
        },
    };

    pub fn new(mine_concentration: f64, seed: Option<u64>) -> CellBuilder {
        let seed = seed.unwrap_or_else(|| Self::get_random_seed());
        let origin = Self::first_valid_start(seed, mine_concentration);
        CellBuilder {
            mine_concentration,
            seed,
            origin,
        }
    }

    pub fn cell(&self, place: PlaceI32) -> Cell {
        let value = Self::cell_value_before_origin(self.seed, self.mine_concentration, PlaceI32 {
            x: place.x + self.origin.x,
            y: place.y + self.origin.y
        });
        let cell = Cell::new(value);
        cell
    }

    fn cell_value_before_origin(seed_u64: u64, mine_concentration: f64, place: PlaceI32) -> CellValue {
        let mut seed = [42; 32];
        seed[ 0..8 ].copy_from_slice(&seed_u64.to_be_bytes());
        seed[ 8..12].copy_from_slice(&place.x .to_be_bytes());
        seed[12..16].copy_from_slice(&place.y .to_be_bytes());

        let mut rng = StdRng::from_seed(seed);

        if mine_concentration > rng.random() {
            CellValue::Mine
        } else {
            CellValue::Empty
        }
    }

    fn first_valid_start(seed: u64, mine_concentration: f64) -> PlaceI32 {
        for x in -500..500 {
            for y in -500..500 {
                let place = PlaceI32 { x, y };
                if Self::is_clear(seed, mine_concentration, place) {
                    return place;
                }
            }
        }
        panic!("cannot find valid start; mine concentration is too high");
    }

    fn is_clear(seed: u64, mine_concentration: f64, place: PlaceI32) -> bool {
        for x in -1..=1 {
            for y in -1..=1 {
                let place = PlaceI32 {
                    x: place.x + x,
                    y: place.y + y,
                };
                if let CellValue::Mine = Self::cell_value_before_origin(seed, mine_concentration, place) {
                    return false;
                }
            }
        }
        true
    }

    fn get_random_seed() -> u64 {
        let mut rng = StdRng::from_os_rng();
        rng.next_u64()
        // ;0xDEADBEEF
    }
}