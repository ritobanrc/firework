use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit, RenderObject, Scene};
use tiny_rng::LcRng;

pub struct BVHNode<'a> {
    next: BVHNodeVariant<'a>,
    aabb: AABB,
}

enum BVHNodeVariant<'a> {
    Leaf(&'a RenderObject<'a>),
    DoubleLeaf(&'a RenderObject<'a>, &'a RenderObject<'a>),
    Branch(Box<BVHNode<'a>>, Box<BVHNode<'a>>),
}

impl<'a> BVHNode<'a> {
    pub fn new(scene: &'a Scene) -> BVHNode<'a> {
        // TODO: proper error handling
        let mut indicies: Vec<usize> = (0..scene.render_objects.len()).collect();
        BVHNode::new_helper(&scene, &mut indicies, 0)
    }

    fn new_helper(scene: &'a Scene, indicies: &mut [usize], depth: usize) -> Self {
        // TODO: Figure out why bounding_box returns an option
        // TODO: Replace all the `expect`s with proper error handling

        indicies.sort_unstable_by(|a, b| {
            let a_box = scene
                .get_object(*a)
                .bounding_box()
                .expect("Bounding Box not found in BVH constructor");
            let b_box = scene
                .get_object(*b)
                .bounding_box()
                .expect("Bounding Box not found in BVH constructor");
            a_box.min[depth % 3]
                .partial_cmp(&b_box.min[depth % 3])
                .expect("Float comparison failed in BVH constructor")
        });

        match indicies {
            &mut [a] => {
                let aabb = scene
                    .get_object(a)
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                BVHNode {
                    next: BVHNodeVariant::Leaf(&scene.get_object(a)),
                    aabb,
                }
            }
            &mut [a, b] => {
                let a_box = scene
                    .get_object(a)
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                let b_box = scene
                    .get_object(b)
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                BVHNode {
                    next: BVHNodeVariant::DoubleLeaf(scene.get_object(a), scene.get_object(b)),
                    aabb: a_box.expand(&b_box),
                }
            }
            l => {
                let (front_half, back_half) = l.split_at_mut(l.len() / 2);
                let left = BVHNode::new_helper(&scene, front_half, depth + 1);
                let right = BVHNode::new_helper(&scene, back_half, depth + 1);

                let left_box = left
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                let right_box = right
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");

                let aabb = left_box.expand(&right_box);
                BVHNode {
                    next: BVHNodeVariant::Branch(Box::new(left), Box::new(right)),
                    aabb,
                }
            }
        }
    }
}

impl Hitable for BVHNode<'_> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        if self.aabb.hit(r, t_min, t_max) {
            match &self.next {
                BVHNodeVariant::Leaf(a) => a.hit(r, t_min, t_max, rand),
                BVHNodeVariant::DoubleLeaf(a, b) => {
                    // Note this is identifical
                    let left_hit = a.hit(r, t_min, t_max, rand);
                    let right_hit = b.hit(r, t_min, t_max, rand);
                    match (left_hit, right_hit) {
                        (None, None) => None,
                        (Some(hit), None) => Some(hit),
                        (None, Some(hit)) => Some(hit),
                        (Some(left_hit), Some(right_hit)) if left_hit.t < right_hit.t => {
                            Some(left_hit)
                        }
                        (Some(_left_hit), Some(right_hit)) => Some(right_hit),
                    }
                }
                BVHNodeVariant::Branch(a, b) => {
                    let left_hit = a.hit(r, t_min, t_max, rand);
                    let right_hit = b.hit(r, t_min, t_max, rand);
                    match (left_hit, right_hit) {
                        (None, None) => None,
                        (Some(hit), None) => Some(hit),
                        (None, Some(hit)) => Some(hit),
                        (Some(left_hit), Some(right_hit)) if left_hit.t < right_hit.t => {
                            Some(left_hit)
                        }
                        (Some(_left_hit), Some(right_hit)) => Some(right_hit),
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}
