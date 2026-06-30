mod app;
mod bench;
mod galaxy;
mod object;
mod quadtree;
mod state;
mod tools;

use {
    crate::{
        app::App,
        galaxy::{SpiralGalaxy, spiral_galaxy},
        quadtree::TreeParams,
        tools::make_orbit_each_other,
    },
    macroquad::{miniquad::window::set_window_size, prelude::*},
    std::f32::consts::PI,
};

const SPEED: f32 = 1.;
const THETA: f32 = 1.;
const MAX_DELTA_TIME: f32 = 0.4;

#[macroquad::main("Barnes-Hut")]
async fn main() {
    set_window_size(800, 800);

    let params = TreeParams {
        max_depth: 9,
        leaf_size: Vec2::new(8., 8.),
        center: Vec2::ZERO,
    };

    let mut galaxy1 = SpiralGalaxy {
        pos: Vec2::new(-170., 0.),
        vel: Vec2::new(0., 0.),
        mass: 500000.,
        max_radius: 160.,
        objects_count: 10000,
        min_radius: 40.,
        sleeves: 4,
        curvature_angle: 5. * PI / 4.,
    };
    let mut galaxy2 = SpiralGalaxy {
        pos: Vec2::new(170., 0.),
        vel: Vec2::new(0., 0.),
        mass: 500000.,
        max_radius: 160.,
        objects_count: 10000,
        min_radius: 40.,
        sleeves: 2,
        curvature_angle: 5. * PI / 4.,
    };

    make_orbit_each_other(&mut galaxy1, &mut galaxy2);

    let mut objects = Vec::new();
    objects.append(&mut spiral_galaxy(galaxy1));
    objects.append(&mut spiral_galaxy(galaxy2));

    App::new(objects, params, THETA, MAX_DELTA_TIME, SPEED)
        .run()
        .await;
}
