// main.rs

use std::thread;
use std::time::{Duration, Instant};

mod player;
mod platform;
mod gamestate;

use gamestate::GameState;

fn main() {
    let mut game_state = GameState::new();
    let target_fps = 10; // Target frame rate
    let target_frame_time = Duration::from_secs_f32(1.0 / target_fps as f32);

    loop {
        let frame_start = Instant::now();

        // Example actions for testing purposes (replace with network input handling)
        game_state.move_player_left(1);
        game_state.move_player_right(2);
        game_state.jump_player(1);

        // Update the game state
        game_state.update();

        // Get and print player positions (for testing purposes)
        for (id, x, y) in game_state.get_player_positions() {
            println!("Player {}: ({:.2}, {:.2})", id, x, y);
        }

        // Maintain the target frame rate
        let frame_duration = frame_start.elapsed();
        if frame_duration < target_frame_time {
            thread::sleep(target_frame_time - frame_duration);
        }

        // Stop player movement after updating (for testing purposes)
        game_state.stop_player(1);
        game_state.stop_player(2);
    }
}
