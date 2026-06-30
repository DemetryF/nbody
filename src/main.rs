mod bench;
mod galaxy;
mod object;
mod quadtree;
mod state;
mod tools;

use {
    crate::{
        galaxy::{SpiralGalaxy, spiral_galaxy},
        quadtree::TreeParams,
        state::State,
        tools::make_orbit_each_other,
    },
    macroquad::{miniquad::window::set_window_size, prelude::*},
    std::{f32::consts::PI, time::Instant},
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
        max_radius: 130.,
        objects_count: 10000,
        min_radius: 40.,
        sleeves: 4,
        curvature_angle: 5. * PI / 4.,
    };
    let mut galaxy2 = SpiralGalaxy {
        pos: Vec2::new(170., 0.),
        vel: Vec2::new(0., 0.),
        mass: 500000.,
        max_radius: 130.,
        objects_count: 10000,
        min_radius: 40.,
        sleeves: 2,
        curvature_angle: 5. * PI / 4.,
    };

    make_orbit_each_other(&mut galaxy1, &mut galaxy2);

    let mut objects = Vec::new();
    objects.append(&mut spiral_galaxy(galaxy1));
    objects.append(&mut spiral_galaxy(galaxy2));

    let mut state = State::new(objects, params);
    state.init(THETA);

    let mut delta_time: f32 = 0.;
    let mut center = Vec2::default();

    loop {
        let frame_start = Instant::now();

        handle_arrows(&mut center, delta_time);

        clear_background(BLACK);
        draw_fps();

        state.update((SPEED * delta_time).min(MAX_DELTA_TIME), THETA);

        for obj in &state.objects {
            draw_circle(
                screen_width() / 2. + center.x + obj.pos.x,
                screen_height() / 2. + center.y + obj.pos.y,
                obj.radius as f32,
                Color::from_rgba(255, 255, 255, 255),
            );
        }

        next_frame().await;

        delta_time = frame_start.elapsed().as_secs_f32();
    }
}

fn handle_arrows(center: &mut Vec2, dt: f32) {
    if is_key_down(KeyCode::Up) {
        center.y += 200. * dt;
    }

    if is_key_down(KeyCode::Down) {
        center.y -= 200. * dt;
    }

    if is_key_down(KeyCode::Left) {
        center.x += 200. * dt;
    }

    if is_key_down(KeyCode::Right) {
        center.x -= 200. * dt;
    }
}
