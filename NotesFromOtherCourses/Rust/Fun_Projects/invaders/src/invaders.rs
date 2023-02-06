use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
};
use rusty_time::timer::Timer;
use std::{cmp::max, time::Duration};


pub struct Invader {
    x: usize,
    y: usize,
    points: u16,
}

// Invader manager or lets call it army of invaders
pub struct Invaders {
    pub army: Vec<Invader>, 
    move_timer: Timer,
    direction: i32
}

// What logic do we need to implement for our invaders?
impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        // For every x and y on the play field
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                // let's fill up army
                if (x > 1) 
                    && (x < NUM_COLS -2) 
                    && (y > 0) 
                    && (y < 9) 
                    && (x % 2 == 0) // only on even coordinates
                    && (y % 2 == 0)
                    {
                        army.push(Invader {x, y, points: 1 });
                    }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1, // right
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            // since the timer is ready we need to reset it:
            self.move_timer.reset();
            // determine that is it time to move downwards
            let mut downwards = false;
            if self.direction == -1 { // we are moving left
                // map every invader to its x && then take the minimum x value
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 { // need to go down
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {  // reached right side -> direction needs to flip
                    self.direction = -1;
                    downwards = true;
                }
            }
            // whenever we move downwards, we have to increase invaders speed
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1; // move invaders downwards
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }

            // return army moved
            return true;
        }
        //return army not moved
        false
    }
    // get if all the enemies were killed
    pub fn all_killed(&self) -> bool {
        self.army.is_empty()
    }
    pub fn reached_bottom(&self) -> bool {
        self.army.iter().map(|invader| invader.y).max().unwrap_or(0) >= NUM_ROWS - 1
    }
    // kill an invader
    pub fn kill_invader_at(&mut self, x: usize, y: usize) -> u16 { // return if at least one killed
        if let Some(index) = self
            .army
            .iter()
            .position(|invader| (invader.x == x) && (invader.y == y)) {
                let points = self.army[index].points;
                self.army.remove(index);
                points
            } else {
                0
            }
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            // half of the time we are going to do an other char to imitate movement
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            }
        }
    }
}