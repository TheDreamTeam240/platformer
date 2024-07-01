// player.rs

use crate::platform::Platform;

pub struct Player {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub width: f32,
    pub height: f32,
}

impl Player {
    // Create a new player instance
    pub fn new(id: usize, x: f32, y: f32) -> Player {
        Player { id, x, y, vx: 0.0, vy: 0.0, width: 1.0, height: 1.0 }
    }

    // Update the player's position and handle collisions
    pub fn update(&mut self, platforms: &Vec<Platform>) {
        self.x += self.vx;
        self.y += self.vy;
        self.vy -= 0.1; // Gravity

        for platform in platforms {
            if self.collides_with(platform) {
                self.resolve_collision(platform);
            }
        }
    }

    // Set the player's velocity
    pub fn set_velocity(&mut self, vx: f32, vy: f32) {
        self.vx = vx;
        self.vy = vy;
    }

    // Check if the player is colliding with a platform
    fn collides_with(&self, platform: &Platform) -> bool {
        self.x < platform.x + platform.width &&
        self.x + self.width > platform.x &&
        self.y < platform.y + platform.height &&
        self.y + self.height > platform.y
    }

    // Resolve the collision with a platform
    fn resolve_collision(&mut self, platform: &Platform) {
        if self.vy > 0.0 { // Falling down
            self.y = platform.y - self.height;
            self.vy = 0.0;
        } else if self.vy < 0.0 { // Jumping up
            self.y = platform.y + platform.height;
            self.vy = 0.0;
        }

        if self.vx > 0.0 { // Moving right
            self.x = platform.x - self.width;
        } else if self.vx < 0.0 { // Moving left
            self.x = platform.x + platform.width;
        }
    }
}
