use std::{
    ops::{Add, Div, Sub},
};

#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }

    pub fn scale(&self, scale: f32) -> Self {
        Vector2 {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    pub fn div(self, divisor: f32) -> Self {
        Vector2 {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }

    pub fn dot(self, other: Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn mul(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }

    pub fn len(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn distance(&self, other: Vector2) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        ((x * x) + (y * y)).sqrt()
    }
}

impl Copy for Vector2 {}

impl Clone for Vector2 {
    fn clone(&self) -> Self {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
