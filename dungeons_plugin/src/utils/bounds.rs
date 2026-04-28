use bevy::prelude::Vec2;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, position: Vec2) -> bool {
        position.x >= self.position.x - self.size.x / 2.0
            && position.x <= self.position.x + self.size.x / 2.0
            && position.y >= self.position.y - self.size.y / 2.0
            && position.y <= self.position.y + self.size.y / 2.0
    }
}

impl Display for Bounds2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bounds2 {{ position: {:?}, size: {:?} }}",
            self.position, self.size
        )
    }
}
