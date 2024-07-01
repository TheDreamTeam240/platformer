// gamestate.rs

use crate::player::Player;
use crate::platform::Platform;

pub struct GameState {
    pub players: Vec<Player>,
    pub platforms: Vec<Platform>,
}

impl GameState {
    pub fn new() -> GameState {
        let players = vec![
            Player::new(1, 1.0, 1.0),
            Player::new(2, 3.0, 1.0),
        ];
        let platforms = vec![
            Platform::new(0.0, 10.0, 20.0, 1.0),
            Platform::new(5.0, 15.0, 10.0, 1.0),
            Platform::new(0.0, 20.0, 20.0, 1.0),
        ];
        GameState { players, platforms }
    }

    pub fn update(&mut self) {
        for player in &mut self.players {
            player.update(&self.platforms);
        }
    }

    pub fn set_player_velocity(&mut self, player_id: usize, vx: f32, vy: f32) {
        if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
            player.set_velocity(vx, vy);
        }
    }

    pub fn jump_player(&mut self, player_id: usize) {
        if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
            if player.vy == 0.0 {
                player.vy = -2.0;
            }
        }
    }

    pub fn move_player_left(&mut self, player_id: usize) {
        self.set_player_velocity(player_id, -1.0, 0.0);
    }

    pub fn move_player_right(&mut self, player_id: usize) {
        self.set_player_velocity(player_id, 1.0, 0.0);
    }

    pub fn stop_player(&mut self, player_id: usize) {
        self.set_player_velocity(player_id, 0.0, 0.0);
    }

    pub fn get_player_positions(&self) -> Vec<(usize, f32, f32)> {
        self.players.iter().map(|p| (p.id, p.x, p.y)).collect()
    }
}
