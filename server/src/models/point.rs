use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: i32
}