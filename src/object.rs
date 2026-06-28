use macroquad::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Object {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub acc_next: Vec2,
    pub mass: f32,
    pub radius: f32,
}
