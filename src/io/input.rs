use clap::Parser;

/// minesweeper on an infinite grid in the terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Input {
    /// fraction of cells that are mines
    #[arg(name = "mine-concentration", short, long, default_value_t = Self::DEFAULT_MINE_CONCENTRATION)]
    pub mine_concentration: f64,
    
    /// seed for the world generator
    #[arg(short, long, hide_default_value = true)]
    pub seed: Option<u64>,
    
    /// width of the visible grid
    #[arg(short = 'W', long, default_value_t = Self::DEFAULT_WIDTH)]
    pub width:  usize,
    
    /// height of the visible grid
    #[arg(short = 'H', long, default_value_t = Self::DEFAULT_HEIGHT)]
    pub height: usize,
}

impl Input {
    pub const DEFAULT_MINE_CONCENTRATION: f64 = 0.2f64;
    pub const DEFAULT_WIDTH:  usize = 16;
    pub const DEFAULT_HEIGHT: usize = 16;
}