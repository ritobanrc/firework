use crate::aabb::AABB;
use crate::objects::{Triangle, TriangleMesh};
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::{RenderObjectInternal, SceneInternal};
use tiny_rng::LcRng;

pub struct BVHNode<'a, T> {
    next: BVHNodeVariant<'a, T>,
    aabb: AABB,
}

enum BVHNodeVariant<'a, T> {
    Leaf(&'a T),
    DoubleLeaf(&'a T, &'a T),
    Branch(Box<BVHNode<'a, T>>, Box<BVHNode<'a, T>>),
}

fn new_helper<'a, A>(
    aggregate: &'a A,
    indicies: &mut [usize],
    depth: usize,
) -> BVHNode<'a, A::BVHType>
where
    A: Aggregate + ?Sized,
    A::BVHType: Hitable,
{
    // TODO: Figure out why bounding_box returns an option
    // TODO: Replace all the `expect`s with proper error handling

    indicies.sort_by(|a, b| {
        let a_box = aggregate.index(*a).bounding_box();
        let b_box = aggregate.index(*b).bounding_box();
        a_box.center()[depth % 3]
            .partial_cmp(&b_box.center()[depth % 3])
            .expect("Float comparison failed in BVH constructor")
    });

    match indicies {
        &mut [a] => {
            let aabb = aggregate.index(a).bounding_box();
            //println!("[Leaf] --  BBOX: {:?}", aabb);
            BVHNode {
                next: BVHNodeVariant::Leaf(aggregate.index(a)),
                aabb,
            }
        }
        &mut [a, b] => {
            let a_box = aggregate.index(a).bounding_box();
            let b_box = aggregate.index(b).bounding_box();
            //println!("[DoubleLeaf] --  LEFT BBOX: {:?} -- RIGHT BBOX: {:?}", a_box, b_box);
            BVHNode {
                next: BVHNodeVariant::DoubleLeaf(aggregate.index(a), aggregate.index(b)),
                aabb: a_box.expand(&b_box),
            }
        }
        l => {
            let (front_half, back_half) = l.split_at_mut(l.len() / 2);
            let left = new_helper(aggregate, front_half, depth + 1);
            let right = new_helper(aggregate, back_half, depth + 1);

            let left_box = left.bounding_box();
            let right_box = right.bounding_box();

            let aabb = left_box.expand(&right_box);
            //println!("[Branch] --  LEFT BBOX: {:?} -- RIGHT BBOX: {:?} -- TOTAL BBOX: {:?}", left_box, right_box, aabb);
            BVHNode {
                next: BVHNodeVariant::Branch(Box::new(left), Box::new(right)),
                aabb,
            }
        }
    }
}

pub trait Aggregate {
    type BVHType;

    fn len(&self) -> usize;
    fn index(&self, index: usize) -> &Self::BVHType;

    fn build_bvh<'a>(&'a self) -> BVHNode<'a, Self::BVHType>
    where
        Self::BVHType: Hitable,
    {
        let mut indicies: Vec<usize> = (0..self.len()).collect();
        new_helper(self, &mut indicies, 0)
    }
}

impl Aggregate for SceneInternal {
    type BVHType = RenderObjectInternal;

    fn len(&self) -> usize {
        self.render_objects.len()
    }

    fn index(&self, index: usize) -> &RenderObjectInternal {
        self.get_object(index)
    }
}

impl<T: Hitable> Hitable for BVHNode<'_, T> {
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

    fn bounding_box(&self) -> AABB {
        self.aabb.clone()
    }
}
