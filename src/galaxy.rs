use {
    crate::object::Object,
    macroquad::prelude::*,
    std::{f32::consts::PI, iter},
};

pub struct SpiralGalaxy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub sleeves: usize,
    pub curvature_angle: f32,
    pub objects_count: usize,
}

pub fn spiral_galaxy(galaxy: SpiralGalaxy) -> Vec<Object> {
    let curvature = galaxy.curvature_angle.powf(-1.) * (galaxy.max_radius / galaxy.min_radius).ln();

    let eccentrity = 0.3;

    (0..galaxy.objects_count)
        .map(|_| {
            let sleeve = rand::gen_range(0, galaxy.sleeves);

            let radius = galaxy.min_radius
                + rand::gen_range::<f32>(0., 1.).powf(0.5)
                    * (galaxy.max_radius - galaxy.min_radius);

            let angle = {
                let mn_hf_ax = radius * f32::sqrt((1. - eccentrity) / (1. + eccentrity));
                let peri = radius * (1. - eccentrity);

                let spiral_point_radius = f32::sqrt(mn_hf_ax * mn_hf_ax + peri * peri / 4.);

                let spiral_point_angle = curvature.powf(-1.)
                    * (spiral_point_radius / galaxy.min_radius).ln()
                    + sleeve as f32 * (2. * PI) / galaxy.sleeves as f32;

                let shift = f32::atan2(2. * mn_hf_ax, peri);

                spiral_point_angle + shift + PI / rand::gen_range::<f32>(3., 4.)
            };

            let u = Vec2::from_angle(angle);
            let pos = galaxy.pos + -u * radius;

            let speed = f32::sqrt(galaxy.mass * (2. / radius - (1. + eccentrity) / radius));
            let local_vel = u.rotate(Vec2::new(0., 1.)) * speed;
            let vel = galaxy.vel + local_vel;

            let mass = rand::gen_range(0.1, 1f32).powf(10.) * 0.01;

            Object {
                pos,
                vel,
                mass,
                radius: 0.35,
                ..Default::default()
            }
        })
        .chain(iter::once(Object {
            mass: galaxy.mass,
            radius: 5.,
            pos: galaxy.pos,
            vel: galaxy.vel,
            ..Default::default()
        }))
        .collect()
}
