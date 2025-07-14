use clap::Parser;

/// minesweeper on an infinite grid in the terminal
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = r#"
Minesweeper on an infinite grid in the terminal.
Keybinds:
    Arrow keys for movement
    Space to reveal
    F to flag
    A to reveal adjacent, non-flagged cells
    R to restart"#,
)]
pub struct Input {
    /// Fraction of cells that are mines [default: 0.2]
    #[arg(
        name = "mine-concentration",
        short, long,
        default_value_t = Self::DEFAULT_MINE_CONCENTRATION,
        hide_default_value = true,
    )]
    pub mine_concentration: f64,
    
    /// Seed for the world generator [default: random]
    #[arg(short, long, hide_default_value = true)]
    pub seed: Option<u64>,

    /// Use gray background
    #[arg(name = "light-mode", short, long, default_value_t = false)]
    pub light_mode: bool,
}

impl Input {
    pub const DEFAULT_MINE_CONCENTRATION: f64 = 0.2f64;
}