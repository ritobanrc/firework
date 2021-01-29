use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use crate::util;
use std::sync::Arc;
use tiny_rng::LcRng;
use ultraviolet::{Vec2, Vec3};

type TriangleIdx = usize;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TriangleMesh {
    indicies: Vec<usize>,
    verts: Vec<Vec3>,
    normals: Option<Vec<Vec3>>,
    uvs: Option<Vec<Vec2>>,
    material: MaterialIdx,
}

impl crate::serde_compat::AsHitable for TriangleMesh {
    fn to_hitable(self: Box<Self>) -> Box<dyn Hitable>
    where
        Self: 'static,
    {
        use crate::bvh::Aggregate;
        let arc: Arc<TriangleMesh> = self.into();
        Box::new(arc.build_bvh())
    }
}

#[typetag::serde]
impl crate::serde_compat::SerializableShape for TriangleMesh {}

impl TriangleMesh {
    /// Creates a new `TriangleMesh` from arrays of data.
    pub fn new(
        verts: Vec<Vec3>,
        indicies: Vec<usize>,
        normals: Option<Vec<Vec3>>,
        uvs: Option<Vec<Vec2>>,
        material: MaterialIdx,
    ) -> Result<TriangleMesh, &'static str> {
        let num_verts = verts.len();
        if let Some(normals) = &normals {
            if normals.len() != num_verts {
                return Err("TriangleMesh::new() -- normals.len() must equal verts.len()");
            }
        }
        if let Some(uvs) = &uvs {
            if uvs.len() != num_verts {
                return Err("TriangleMesh::new() -- uvs.len() must equal verts.len()");
            }
        }

        Ok(TriangleMesh {
            indicies,
            verts,
            normals,
            uvs,
            material,
        })
    }

    /// Translates every vertex in the `TriangleMesh` by `pos`
    pub fn translate(mut self, pos: Vec3) -> Self {
        for vert in &mut self.verts {
            *vert += pos;
        }
        self
    }

    /// Returns the verticies for a given triangle
    pub fn get_triangle_verts(&self, idx: TriangleIdx) -> [Vec3; 3] {
        let base_idx = 3 * idx;
        [
            self.verts[self.indicies[base_idx]],
            self.verts[self.indicies[base_idx + 1]],
            self.verts[self.indicies[base_idx + 2]],
        ]
    }

    /// Returns the normal for each vertex of a triangle, if supplied
    pub fn get_triangle_normals(&self, idx: TriangleIdx) -> Option<[Vec3; 3]> {
        if let Some(normals) = &self.normals {
            let base_idx = 3 * idx;
            Some([
                normals[self.indicies[base_idx]],
                normals[self.indicies[base_idx + 1]],
                normals[self.indicies[base_idx + 2]],
            ])
        } else {
            None
        }
    }

    /// Returns the UV supplied coordinates for a given triangle (if supplied), or
    /// (0, 0), (0, 1), and (1, 0) for each of the 3 verticies
    pub fn get_triangle_uvs(&self, idx: TriangleIdx) -> [Vec2; 3] {
        if let Some(uv) = &self.uvs {
            let base_idx = 3 * idx;
            [
                uv[self.indicies[base_idx]],
                uv[self.indicies[base_idx + 1]],
                uv[self.indicies[base_idx + 2]],
            ]
        } else {
            [Vec2::zero(), Vec2::unit_x(), Vec2::unit_y()]
        }
    }

    pub fn num_verts(&self) -> usize {
        self.verts.len()
    }

    pub fn num_tris(&self) -> usize {
        self.indicies.len() / 3
    }
}

pub struct Triangle {
    pub(crate) mesh: Arc<TriangleMesh>, // I don't like this. This feel "object oriented". Also, there should be a way to easily swap out Arc for Rc if the user isn't using multithreading
    pub(crate) index: TriangleIdx,
}

impl Triangle {
    pub fn new(mesh: Arc<TriangleMesh>, index: TriangleIdx) -> Triangle {
        Triangle { mesh, index }
    }
}

//impl std::ops::Deref for Triangle {
//type Target = Triangle;
//fn deref(&self) -> &Self::Target {
//self
//}
//}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let [p0, p1, p2] = self.mesh.get_triangle_verts(self.index);
        // M = SPT
        // TODO: Optimize this to use `Wec3`s
        // Translate the verticies to the ray origin
        let mut p0t = p0 - *r.origin();
        let mut p1t = p1 - *r.origin();
        let mut p2t = p2 - *r.origin();

        // Permute verticies and ray direction
        let d = *r.direction();
        let kz = util::max_component_idx(d);
        let kx = (kz + 1) % 3;
        let ky = (kx + 1) % 3;

        let d = Vec3::new(d[kx], d[ky], d[kz]);
        p0t = Vec3::new(p0t[kx], p0t[ky], p0t[kz]);
        p1t = Vec3::new(p1t[kx], p1t[ky], p1t[kz]);
        p2t = Vec3::new(p2t[kx], p2t[ky], p2t[kz]);

        let sx = -d.x / d.z;
        let sy = -d.y / d.z;
        let sz = 1. / d.z;

        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;

        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;

        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;

        let e0 = p1t.x * p2t.y - p1t.y * p2t.x;
        let e1 = p2t.x * p0t.y - p2t.y * p0t.x;
        let e2 = p0t.x * p1t.y - p0t.y * p1t.x;

        if (e0 < 0. || e1 < 0. || e2 < 0.) && (e0 > 0. || e1 > 0. || e2 > 0.) {
            return None;
        }
        let det = e0 + e1 + e2;
        if det == 0. {
            return None;
        }

        p0t.z *= sz;
        p1t.z *= sz;
        p2t.z *= sz;

        let t_scaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;
        if det < 0. && (t_scaled >= t_min * det || t_scaled < t_max * det) {
            return None;
        } else if det > 0. && (t_scaled <= t_min * det || t_scaled > t_max * det) {
            return None;
        }

        let inv_det = 1. / det;
        let b0 = e0 * inv_det;
        let b1 = e1 * inv_det;
        let b2 = e2 * inv_det;

        let t = t_scaled * inv_det;
        let point = b0 * p0 + b1 * p1 + b2 * p2;
        let uvs = self.mesh.get_triangle_uvs(self.index);
        let uv = b0 * uvs[0] + b1 * uvs[1] + b2 * uvs[2];

        let normal = if let Some(normals) = self.mesh.get_triangle_normals(self.index) {
            (b0 * normals[0] + b1 * normals[1] + b2 * normals[2]).normalized()
        } else {
            (p0 - p2).cross(p1 - p2)
        };

        Some(RaycastHit {
            t,
            point,
            normal,
            material: self.mesh.material,
            uv,
        })
    }

    fn bounding_box(&self) -> AABB {
        let [p0, p1, p2] = self.mesh.get_triangle_verts(self.index);

        let mut aabb = AABB::from_two_points(p0, p1).expand_to_point(p2);
        let size = (aabb.max - aabb.min).abs();

        // Make sure the box has non-zero volume
        if size.x < 0.001 {
            aabb.min.x -= 0.001;
            aabb.max.x += 0.001;
        }
        if size.y < 0.001 {
            aabb.min.y -= 0.001;
            aabb.max.y += 0.001;
        }
        if size.z < 0.001 {
            aabb.min.z -= 0.001;
            aabb.max.z += 0.001;
        }

        aabb
    }
}
