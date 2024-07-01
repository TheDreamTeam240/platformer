// platform.rs

pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Platform {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Platform {
        Platform { x, y, width, height }
    }
}
