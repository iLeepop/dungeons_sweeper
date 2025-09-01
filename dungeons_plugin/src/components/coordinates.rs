use bevy::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl Add for Coordinates {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Self;

    fn add(self, (x, y): (i8, i8)) -> Self::Output {
        let x = ((self.x as i16) + x as i16) as u32;
        let y = ((self.y as i16) + y as i16) as u32;
        Self { x, y }
    }
}

impl Sub for Coordinates {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}