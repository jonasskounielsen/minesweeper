use clap::{Parser, ValueEnum};

/// Minesweeper on an infinite grid in the terminal.
#[derive(Parser, Debug)]
#[command(
    about,
    long_about = r#"
Minesweeper on an infinite grid in the terminal.
Keybinds:
    Arrow keys for movement
    Space to reveal
      to flag
      to reveal adjacent, non-flagged cells
      to restart"#,
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
    #[arg(
        name = "seed",
        short, long,
    )]
    pub seed: Option<u64>,

    /// Keybinds [default: vim]
    #[arg(
        name = "keybinds",
        short, long,
        value_enum,
        default_value_t = Keybinds::Vim,
        hide_default_value = true,
    )]
    pub keybinds: Keybinds,

    /// Use authentic gray background color [default: off]
    #[arg(
        name = "light-mode",
        short, long,
        default_value_t = false,
    )]
    pub light_mode: bool,

    /// Print version
    #[arg(
        name = "version",
        short, long,
    )]
    pub print_version: bool,
}

impl Input {
    pub const DEFAULT_MINE_CONCENTRATION: f64 = 0.2f64;
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Keybinds {
    Vim,
    Wasd,
    Arrows,
}
