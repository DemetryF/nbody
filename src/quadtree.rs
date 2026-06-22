use {
    crate::object::Object,
    macroquad::prelude::*,
    std::{iter, mem},
};

pub struct Tree {
    pub root: Node,
    pub params: TreeParams,
}

impl Tree {
    pub fn new(params: TreeParams) -> Self {
        let root = Node {
            center: params.center,
            depth: 0,
            mass: 0.,
            mass_x_pos: Vec2::default(),
            kind: NodeKind::Leaf(Leaf::default()),
        };

        Self { root, params }
    }

    pub fn build(mut self, objects: &[Object]) -> Self {
        for &obj in objects {
            self.root.push(obj, self.params);
        }

        self
    }
}

#[derive(Clone, Copy)]
pub struct TreeParams {
    pub max_depth: i32,
    pub leaf_size: Vec2,
    pub center: Vec2,
}

impl TreeParams {
    pub fn sector_size_at_depth(&self, depth: i32) -> Vec2 {
        self.leaf_size * 2f32.powi(self.max_depth - depth)
    }
}

pub struct Node {
    pub center: Vec2,
    pub depth: i32,
    pub mass: f32,
    pub mass_x_pos: Vec2,
    pub kind: NodeKind,
}

impl Node {
    pub fn mass_center(&self) -> Vec2 {
        self.mass_x_pos / self.mass
    }

    pub fn push(&mut self, obj: Object, params: TreeParams) {
        self.mass += obj.mass;
        self.mass_x_pos += obj.mass * obj.pos;

        let local_pos = obj.pos - self.center;
        let local_idx = to_local_idx(local_pos);

        match &mut self.kind {
            NodeKind::Leaf(leaf) if self.depth == params.max_depth || leaf.objects.is_empty() => {
                leaf.objects.push(obj);
            }

            NodeKind::Leaf(leaf) => {
                // take objects and remove mass props
                let objects = mem::take(&mut leaf.objects);
                self.mass = 0.;
                self.mass_x_pos = Vec2::default();

                // transform leaf to branch
                self.kind = NodeKind::Branch(Branch::default());

                // push objects back
                let objects = objects.iter().copied().chain(iter::once(obj));

                for obj in objects {
                    self.push(obj, params);
                }
            }

            NodeKind::Branch(branch) => {
                let child = branch.children[local_idx].get_or_insert_with(|| {
                    let depth = self.depth + 1;
                    let center =
                        self.center + params.sector_size_at_depth(depth) / 2. * local_pos.signum();

                    Box::new(Node {
                        center,
                        depth,
                        mass: 0.,
                        mass_x_pos: Vec2::default(),
                        kind: NodeKind::Leaf(Leaf::default()),
                    })
                });

                child.push(obj, params);
            }
        }
    }
}

pub enum NodeKind {
    Branch(Branch),
    Leaf(Leaf),
}

#[derive(Default)]
pub struct Branch {
    pub children: [Option<Box<Node>>; 4],
}

#[derive(Default)]
pub struct Leaf {
    pub objects: Vec<Object>,
}

pub fn to_local_idx(vec: Vec2) -> usize {
    let vec2 = vec.signum();

    match (vec2.x, vec2.y) {
        (-1., 1.) => 0,
        (1., 1.) => 1,
        (1., -1.) => 2,
        (-1., -1.) => 3,

        _ => unreachable!("{vec}"),
    }
}
