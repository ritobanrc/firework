use ultraviolet::{Vec3, Vec2};

struct TriangleMesh {
    indicies: Vec<usize>,
    verts: Vec<Vec3>,
    normals: Option<Vec<Vec3>>,
    uvs: Option<Vec<Vec2>>,
}
