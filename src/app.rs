use {
    crate::{object::Object, quadtree::TreeParams, state::State},
    macroquad::prelude::*,
    std::time::Instant,
};

pub struct App {
    center: Vec2,
    delta_time: f32,
    max_dt: f32,
    theta: f32,
    speed: f32,
    state: State,
}

impl App {
    pub fn new(
        objects: Vec<Object>,
        tree_params: TreeParams,
        theta: f32,
        max_dt: f32,
        speed: f32,
    ) -> Self {
        let mut state = State::new(objects, tree_params);

        state.init(theta);

        Self {
            center: Vec2::default(),
            delta_time: f32::default(),
            max_dt,
            theta,
            speed,
            state,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let frame_start = Instant::now();

            self.update().await;

            self.delta_time = frame_start.elapsed().as_secs_f32();
        }
    }

    async fn update(&mut self) {
        self.state
            .update((self.speed * self.delta_time).min(self.max_dt), self.theta);

        self.handle_events();

        self.clear();
        self.draw_objects();

        next_frame().await;
    }

    fn handle_events(&mut self) {
        self.handle_movement();
        self.handle_change_speed();
    }

    fn handle_movement(&mut self) {
        if is_key_down(KeyCode::LeftShift) {
            return;
        }

        if is_key_down(KeyCode::Up) {
            self.center.y += 200. * self.delta_time;
        }

        if is_key_down(KeyCode::Down) {
            self.center.y -= 200. * self.delta_time;
        }

        if is_key_down(KeyCode::Left) {
            self.center.x += 200. * self.delta_time;
        }

        if is_key_down(KeyCode::Right) {
            self.center.x -= 200. * self.delta_time;
        }
    }

    fn handle_change_speed(&mut self) {
        if !is_key_down(KeyCode::LeftShift) {
            return;
        }

        if is_key_down(KeyCode::Left) {
            self.speed /= 1.1;
        }

        if is_key_down(KeyCode::Right) {
            self.speed *= 1.1;
        }
    }

    fn clear(&mut self) {
        clear_background(BLACK);
    }

    fn draw_objects(&mut self) {
        for obj in &self.state.objects {
            draw_circle(
                screen_width() / 2. + self.center.x + obj.pos.x,
                screen_height() / 2. + self.center.y + obj.pos.y,
                obj.radius as f32,
                Color::from_rgba(215, 215, 255, 255),
            );
        }
    }
}
