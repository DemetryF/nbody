mod galaxy;
mod object;
mod quadtree;
mod state;

use {
    crate::{
        galaxy::{SpiralGalaxy, spiral_galaxy},
        quadtree::TreeParams,
        state::State,
    },
    macroquad::{miniquad::window::set_window_size, prelude::*},
    std::{f32::consts::PI, time::Instant},
};

const SPEED: f32 = 2.;
const THETA: f32 = 0.8;

#[macroquad::main("Barnes-Hut")]
async fn main() {
    set_window_size(800, 800);

    let params = TreeParams {
        max_depth: 9,
        leaf_size: Vec2::new(8., 8.),
        center: Vec2::ZERO,
    };

    let mut galaxy1 = spiral_galaxy(SpiralGalaxy {
        pos: Vec2::new(0., 0.),
        vel: Vec2::new(0., 0.),
        mass: 500000.,
        max_radius: 150.,
        objects_count: 10000,
        min_radius: 50.,
        sleeves: 2,
        curvature_angle: 11. * PI / 4.,
    });

    let mut galaxy2 = spiral_galaxy(SpiralGalaxy {
        pos: Vec2::new(200., 0.),
        vel: Vec2::new(0., -43.),
        mass: 30000.,
        max_radius: 90.,
        objects_count: 3000,
        min_radius: 20.,
        sleeves: 2,
        curvature_angle: 3. * PI / 4.,
    });

    let mut objects = Vec::new();
    objects.append(&mut galaxy1);
    objects.append(&mut galaxy2);

    let mut state = State::new(objects, params);

    let mut delta_time: f32 = 0.;

    loop {
        let frame_start = Instant::now();

        clear_background(BLACK);
        draw_fps();

        state.update(SPEED * delta_time.min(0.02f32), THETA);

        for obj in &state.objects {
            draw_circle(
                screen_width() / 2. + obj.pos.x,
                screen_height() / 2. + obj.pos.y,
                obj.radius,
                Color::from_rgba(255, 255, 255, 255),
            );
        }

        next_frame().await;

        delta_time = frame_start.elapsed().as_secs_f32();
    }
}
