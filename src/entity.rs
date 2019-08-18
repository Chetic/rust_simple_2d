#[derive(Copy, Clone)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub rotation: f64
}

impl Entity {
    pub fn new(x: f64, y: f64, size: f64) -> Entity {
        Entity { x, y, size, rotation: 0.0 }
    }
}
