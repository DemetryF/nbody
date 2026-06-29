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

    let eccentricity = 0.4;

    (1..galaxy.objects_count)
        .map(|_| {
            let sleeve = rand::gen_range(0, galaxy.sleeves);

            let radius = galaxy.min_radius
                + rand::gen_range::<f32>(0., 1.).powf(0.5)
                    * (galaxy.max_radius - galaxy.min_radius);

            let mn_hf_ax = radius * f32::sqrt((1. - eccentricity) / (1. + eccentricity));
            let peri = radius * (1. - eccentricity * eccentricity);
            let focal = mn_hf_ax * (1. - eccentricity * eccentricity).sqrt();

            let ellipse_angle = {
                let spiral_point_radius = f32::sqrt(mn_hf_ax * mn_hf_ax + peri * peri / 4.);

                let spiral_point_angle = curvature.powf(-1.)
                    * (spiral_point_radius / galaxy.min_radius).ln()
                    + sleeve as f32 * (2. * PI) / galaxy.sleeves as f32;

                let shift = f32::atan2(2. * mn_hf_ax, peri);

                spiral_point_angle + shift //+ PI / rand::gen_range::<f32>(5., 7.)
            };

            let angle = rand::gen_range(0., 2. * PI);

            let r = focal / (1.0 + eccentricity * angle.cos());

            let periapsis = Vec2::from_angle(ellipse_angle);
            let transversal = -periapsis.rotate(Vec2::Y);

            let radial = periapsis * angle.cos() + transversal * angle.sin();
            let tangential = -periapsis * angle.sin() + transversal * angle.cos();

            let local_pos = radial * r;
            let pos = galaxy.pos + local_pos;

            let local_vel = (galaxy.mass / focal).sqrt()
                * (eccentricity * angle.sin() * radial
                    + (1.0 + eccentricity * angle.cos()) * tangential);

            let vel = galaxy.vel + local_vel;

            let mass = rand::gen_range(0.1, 1f32).powf(10.) * 0.01;

            Object {
                pos,
                vel,
                mass,
                radius: 0.5,
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
