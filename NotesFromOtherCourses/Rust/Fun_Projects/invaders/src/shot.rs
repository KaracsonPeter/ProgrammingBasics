use crate::frame::{Drawable, Frame};
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer, // to keep treaxk of your movment
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50), // go upwords one cell every ms
        }
    }
    // update timer
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);  // counting down the timer by delta
        if self.timer.ready && !self.exploding {  // bullet is moving
            if self.y > 0 { // bounce check
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);  // explode time
    }
    // Does the bullet exploded?
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || self.y == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };
    }
}