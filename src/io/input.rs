use clap::Parser;

/// minesweeper on an infinite grid in the terminal
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = r#"
minesweeper on an infinite grid in the terminal
keybinds:
    arrow keys for movement
    space to reveal
    f to flag
    a to reveal adjacent
    r to restart"#,
)]
pub struct Input {
    /// fraction of cells that are mines
    #[arg(name = "mine-concentration", short, long, default_value_t = Self::DEFAULT_MINE_CONCENTRATION)]
    pub mine_concentration: f64,
    
    /// seed for the world generator (defaults to random)
    #[arg(short, long, hide_default_value = true)]
    pub seed: Option<u64>,
}

impl Input {
    pub const DEFAULT_MINE_CONCENTRATION: f64 = 0.2f64;
}