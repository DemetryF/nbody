use {
    crate::{
        object::Object,
        quadtree::{Node, NodeKind, Tree, TreeParams, to_local_idx},
    },
    macroquad::prelude::*,
    std::thread,
};

pub struct State {
    pub objects: Vec<Object>,
    params: TreeParams,
}

impl State {
    pub fn new(objects: Vec<Object>, params: TreeParams) -> Self {
        Self { objects, params }
    }

    pub fn update(&mut self, delta_time: f32, theta: f32) {
        let tree = Tree::new(self.params).build(&self.objects);

        thread::scope(|scope| {
            let tree = &tree;

            let chunk_size = self.objects.len() / 24;

            for chunk in self.objects.chunks_mut(chunk_size) {
                scope.spawn(move || {
                    for obj in chunk.iter_mut() {
                        let force = calculate_force(*obj, &tree.root, tree.params, theta, true);

                        obj.acc = force / obj.mass;
                        obj.vel += obj.acc * delta_time;
                        obj.pos += obj.vel * delta_time;
                    }
                });
            }
        })
    }
}

fn calculate_force(
    obj: Object,
    node: &Node,
    params: TreeParams,
    theta_threshold: f32,
    belongs: bool,
) -> Vec2 {
    let mass = if belongs {
        node.mass - obj.mass
    } else {
        node.mass
    };

    let mass_center = if belongs {
        (node.mass_x_pos - obj.pos * obj.mass) / (node.mass - obj.mass)
    } else {
        node.mass_center()
    };

    let theta = {
        let size = params.sector_size_at_depth(node.depth).max_element();
        let dist = (mass_center - obj.pos).length();

        size / dist
    };

    if theta < theta_threshold && node.mass != obj.mass {
        let dist = mass_center - obj.pos;
        let force = dist.normalize() * obj.mass * mass / dist.length_squared();

        return force;
    }

    match &node.kind {
        NodeKind::Branch(branch) => {
            let local_idx = to_local_idx(obj.pos - node.center);

            branch
                .children
                .iter()
                .enumerate()
                .filter_map(|(idx, node)| node.as_ref().map(|node| (idx, node)))
                .fold(Vec2::default(), |sum_force, (idx, node)| {
                    let belongs = belongs && local_idx == idx;

                    let force = calculate_force(obj, &node, params, theta_threshold, belongs);

                    sum_force + force
                })
        }

        NodeKind::Leaf(leaf) => leaf
            .objects
            .iter()
            .filter(|&&another_obj| another_obj.pos != obj.pos)
            .fold(Vec2::default(), |sum_force, another_obj| {
                let dist = another_obj.pos - obj.pos;
                let force = dist.normalize() * obj.mass * another_obj.mass / dist.length_squared();

                sum_force + force
            }),
    }
}
