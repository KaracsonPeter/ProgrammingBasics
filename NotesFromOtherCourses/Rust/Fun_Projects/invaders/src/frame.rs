use crate::{NUM_COLS, NUM_ROWS};
// Use a type aliace, cause our frame is going to be a vector of vectors of borrowed static string slices 
pub type Frame = Vec<Vec<& 'static str>>;

// Generate a new Frame for us
pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);

    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

// Everithing that we want to see is going to be able to draw itself into the frame.
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}