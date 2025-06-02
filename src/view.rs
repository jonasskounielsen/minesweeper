use super::Grid;

mod matrix;

pub enum ViewCell {
    Unrevealed,
    Flagged,
    Clear,
    Mine,
    One, Two, Three, Four,
    Five, Six, Seven, Eight,
}

pub struct View<'a> {
    pub grid: &'a mut Grid,

}