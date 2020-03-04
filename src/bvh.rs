use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, HitableList, RaycastHit};
use tiny_rng::LcRng;

pub struct BVHNode<'a> {
    next: BVHNodeVariant<'a>,
    aabb: AABB,
}

enum BVHNodeVariant<'a> {
    Leaf(&'a (dyn Hitable + Sync)),
    DoubleLeaf(&'a (dyn Hitable + Sync), &'a (dyn Hitable + Sync)),
    Branch(Box<BVHNode<'a>>, Box<BVHNode<'a>>),
}

impl<'a> BVHNode<'a> {
    pub fn new(list: &'a mut HitableList) -> Self {
        BVHNode::new_helper(list.list_mut(), 0)
    }

    fn new_helper(list: &'a mut [Box<dyn Hitable + Sync>], depth: usize) -> Self {
        // TODO: Figure out why bounding_box returns an option
        // TODO: Replace all the `expect`s with proper error handling
        match depth % 3 {
            0 => list.sort_unstable_by(|a, b| {
                let a_box = a
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                let b_box = b
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                a_box
                    .min
                    .x
                    .partial_cmp(&b_box.min.x)
                    .expect("Float comparison failed in BVH constructor")
            }),
            1 => list.sort_unstable_by(|a, b| {
                let a_box = a
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                let b_box = b
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                a_box
                    .min
                    .y
                    .partial_cmp(&b_box.min.y)
                    .expect("Float comparison failed in BVH constructor")
            }),
            2 => list.sort_unstable_by(|a, b| {
                let a_box = a
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                let b_box = b
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                a_box
                    .min
                    .x
                    .partial_cmp(&b_box.min.x)
                    .expect("Float comparison failed in BVH constructor")
            }),
            _ => unreachable!(),
        };

        match list {
            [a] => {
                let aabb = a
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                BVHNode {
                    next: BVHNodeVariant::Leaf(list[0].as_ref()),
                    aabb,
                }
            }
            [a, b] => {
                let a_box = a
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                let b_box = b
                    .bounding_box()
                    .expect("Bounding Box not found in BVH constructor");
                BVHNode {
                    next: BVHNodeVariant::DoubleLeaf(list[0].as_ref(), list[1].as_ref()),
                    aabb: a_box.expand(&b_box),
                }
            }
            l => {
                let (front_half, back_half) = l.split_at_mut(l.len() / 2);
                let left = BVHNode::new_helper(front_half, depth + 1);
                let right = BVHNode::new_helper(back_half, depth + 1);

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
