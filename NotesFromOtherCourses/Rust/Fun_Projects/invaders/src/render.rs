use crate::frame::Frame;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

// We will only render what changed, therefore we need a lastframe too
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force_rendering: bool) {
    if force_rendering {
        // Queue up a bunch of commands to the terminal and flush them out all at once. (Crush if an error occures (unwrap()))
        // set background color
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();  
        // clear terminal
        stdout.queue(Clear(ClearType::All)).unwrap();
        // set background color
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();  
    }
    // Iterate through our frame and draw whatever changed
    for (index_x, col) in curr_frame.iter().enumerate() {
        for (index_y, s) in col.iter().enumerate() {
            if *s != last_frame[index_x][index_y] || force_rendering {
                stdout.queue(MoveTo(index_x as u16, index_y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}