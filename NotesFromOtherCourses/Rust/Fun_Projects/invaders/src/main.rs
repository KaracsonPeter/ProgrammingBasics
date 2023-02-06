use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    //level::Level,
    //menu::Menu,
    player::Player,
    render,
    //score::Score,
};

// Return a result of our program. We do not care about success, but for error, we are gonna box up dynamic error treat object.
fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();  // get access to std out
    terminal::enable_raw_mode()?;  // enable raw mode so we can get kexboard input as it occurs. '?' -> program will crush if there is an error
    stdout.execute(EnterAlternateScreen)?;  // Let's enter our alternate screen, with en extension that crossterm provides on std called "execute". (vim also use this to get 2 available screens on the terminal)
    stdout.execute(Hide)?;  // Hide the cursor

    // Let's make a render loop in a separate thread!
    // Set up communication channels between threads
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame(); // init
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(returned_frame) => returned_frame,
                Err(_) => break, // in case of not getting a frame, break out the thread
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // input handling
            // poll for inputs
        while event::poll(Duration::default())? {
            // we only care about key events
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => { 
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}  // If any other key is pressed ignore it.
                };
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) > 0 {
            audio.play("explode");
        }

        // Draw & render
        // draw anything that implements the drawable trait (like the shared pointer stuff in C++ ... cool)
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame); // ignor errors until other thread did not set up
        // sleep this thread cause is would be much faster then the other one
        thread::sleep(Duration::from_millis(1));

        // Win or Lose?
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Cleanup
    drop(render_tx); // now join won't wait forever cause we generated an error in the thread and it breaked
    audio.wait();
    render_handle.join().unwrap();
    // Reverse our terminal settings
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
