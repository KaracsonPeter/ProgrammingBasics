// You can add modules (named after the file) to your lib
pub mod frame; // we wantt to have some frame logic for every frame
pub mod render; // Add another module for rendering our frame out to the terminal
pub mod player;
pub mod shot;
pub mod invaders;

// some const
pub const NUM_ROWS: usize = 20;  // num of rows in the game
pub const NUM_COLS: usize = 40;

