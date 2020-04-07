use ultraviolet::Vec3;

struct TriangleMesh {
    indicies: Vec<usize>,
    verts: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<(f32, f32)>,
}
