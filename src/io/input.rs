use clap::Parser;

/// minesweeper on an infinite grid in the terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Input {
    /// fraction of cells that are mines
    #[arg(name = "mine-concentration", short, long, default_value_t = Self::DEFAULT_MINE_CONCENTRATION)]
    pub mine_concentration: f64,
    
    /// seed for the world generator [default: 0xDEADBEEF]
    #[arg(short, long, default_value_t = Self::DEFAULT_SEED, hide_default_value = true)]
    pub seed: u64,
    
    /// width of the visible grid
    #[arg(short = 'W', long, default_value_t = Self::DEFAULT_WIDTH)]
    pub width:  i32,
    
    /// height of the visible grid
    #[arg(short = 'H', long, default_value_t = Self::DEFAULT_HEIGHT)]
    pub height: i32,
}

impl Input {
    pub const DEFAULT_MINE_CONCENTRATION: f64 = 0.2f64;
    pub const DEFAULT_SEED: u64 = 0xDEADBEEF;
    pub const DEFAULT_WIDTH:  i32 = 16;
    pub const DEFAULT_HEIGHT: i32 = 16;
}