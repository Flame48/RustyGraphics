use std::{ fs, path::Path };

use crate::scene::math::matrix::{ Matrix, RowMat, Transform };

// Will store vertex data such as UV coordinates, color, etc.
#[derive(Clone, Copy)]
pub struct VertexData {}

impl VertexData {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub verts: Matrix<3, 4>,
    pub vertex_normals: Matrix<3, 4>,
    pub vertex_data: [VertexData; 3],
}

impl Triangle {
    pub fn new(p1: [f32; 3], p2: [f32; 3], p3: [f32; 3]) -> Self {
        let to_homogeneous = |p: [f32; 3]| [p[0], p[1], p[2], 1.0];
        Self {
            verts: Matrix {
                data: [to_homogeneous(p1), to_homogeneous(p2), to_homogeneous(p3)],
            },
            vertex_normals: Matrix::<3, 4>::new(),
            vertex_data: [VertexData::new(); 3],
        }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        self.transform_geometry(transform);
        self.transform_normals(transform);
        Self {
            verts: self.verts,
            vertex_normals: self.vertex_normals,
            vertex_data: [VertexData::new(); 3],
        }
    }

    pub fn transform_geometry(&self, transform: Transform) -> Self {
        let mut res = self.clone();
        res.transform_geometry_mut(transform);
        res
    }

    pub fn transform_normals(&self, transform: Transform) -> Self {
        let mut res = self.clone();
        res.transform_normals_mut(transform);
        res
    }

    pub fn transform_mut(&mut self, transform: Transform) {
        self.transform_geometry_mut(transform);
        self.transform_normals_mut(transform);
    }

    pub fn transform_geometry_mut(&mut self, transform: Transform) {
        self.verts = self.verts * transform.forward;
        self.verts.normalize_homogenous_mut();
    }

    pub fn transform_normals_mut(&mut self, transform: Transform) {
        let transformed_vertex_normals = self.vertex_normals * transform.forward;
        for i in 0..3 {
            let n = transformed_vertex_normals.row_mat(i).to_uniform().norm_row();
            self.vertex_normals.data[i] = [n.x(), n.y(), n.z(), 0.0];
        }
    }

    fn compute_face_normal(&self) -> RowMat<4> {
        let v0 = self.verts.row_mat(0).to_uniform();
        let v1 = self.verts.row_mat(1).to_uniform();
        let v2 = self.verts.row_mat(2).to_uniform();
        (v1 - v0)
            .cross(v2 - v0)
            .norm_row()
            .to_homogenous()
    }

    fn use_computed_normals(&mut self) {
        let norm = self.compute_face_normal();
        self.vertex_normals = Matrix::<3, 4>::from_data([
            norm.serial_row(),
            norm.serial_row(),
            norm.serial_row(),
        ]);
    }

    pub fn center(&self) -> RowMat<3> {
        let vs = self.verts.to_uniform();

        let v1 = vs.row_mat(0);
        let v2 = vs.row_mat(1);
        let v3 = vs.row_mat(2);

        return (v1 + v2 + v3) / 3.0;
    }
}

#[derive(Clone)]
pub struct Mesh {
    tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle>) -> Self {
        Self { tris }
    }

    pub fn import(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        match path.extension().and_then(|e| e.to_str()) {
            Some("obj") => Self::import_obj(path),
            Some("stl") => Self::import_stl(path),
            _ => Err("Unrecognized file estension for mesh".into()),
        }
    }

    pub fn import_obj(path: impl AsRef<Path>) -> Result<Self, String> {
        let Ok(contents) = fs::read_to_string(path) else {
            return Err(format!("Unable to open file"));
        };
        Self::parse_obj(&contents)
    }

    pub fn import_stl(path: impl AsRef<Path>) -> Result<Self, String> {
        let Ok(contents) = fs::read(path) else {
            return Err(format!("Unable to open file"));
        };
        Self::parse_stl(&contents)
    }

    fn parse_row_mat<'a, const N: usize>(
        from: &mut impl Iterator<Item = &'a str>
    ) -> Result<RowMat<N>, String> {
        let mut out = RowMat::<N>::new();

        for i in 0..N {
            let Some(tok) = from.next() else {
                return Err(format!("Unable to get float from line"));
            };
            let Ok(value) = tok.parse() else {
                return Err(format!("Unable to get float from line"));
            };
            out.set(0, i, value);
        }

        Ok(out)
    }

    fn parse_obj_face_vertex_indices(from: &str) -> (Option<usize>, Option<usize>, Option<usize>) {
        let mut tokens = from.split('/');

        let vi = tokens.next().and_then(|s|
            s
                .parse::<usize>()
                .ok()
                .and_then(|x| if x < 1 { None } else { Some(x - 1) })
        );
        let ti = tokens.next().and_then(|s|
            s
                .parse::<usize>()
                .ok()
                .and_then(|x| if x < 1 { None } else { Some(x - 1) })
        );
        let ni = tokens.next().and_then(|s|
            s
                .parse::<usize>()
                .ok()
                .and_then(|x| if x < 1 { None } else { Some(x - 1) })
        );
        return (vi, ti, ni);
    }

    fn parse_obj(obj: &str) -> Result<Self, String> {
        let mut vertices = Vec::<RowMat<3>>::new();
        let mut normals = Vec::<RowMat<3>>::new();
        let mut tris = Vec::<Triangle>::new();

        for line in obj.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut tokens = line.split_whitespace();
            let Some(line_type) = tokens.next() else {
                return Err(format!("Unable to parse line type"));
            };

            match line_type {
                "v" => {
                    // Parse vertex
                    vertices.push(Self::parse_row_mat::<3>(&mut tokens)?);
                }
                "vn" => {
                    // Parse vertex
                    normals.push(Self::parse_row_mat::<3>(&mut tokens)?);
                }
                "f" => {
                    // Parse triangle
                    let mut indices = Vec::<(usize, Option<usize>)>::new();
                    while let Some(vertex_info_s) = tokens.next() {
                        let (Some(vi), _, ni) =
                            Self::parse_obj_face_vertex_indices(vertex_info_s) else {
                            continue;
                        };
                        indices.push((vi, ni));
                    }

                    if indices.len() < 3 {
                        return Err(format!("Triangle has less than 3 vertices"));
                    }

                    // Note that indices can contain more than 3 vertices. As such we perform fan interpolaton.
                    for i in 1..indices.len() - 1 {
                        let (v0, n0) = indices[0];
                        let (v1, n1) = indices[i];
                        let (v2, n2) = indices[i + 1];

                        let get_v = |vi: usize|
                            vertices.get(vi).ok_or_else(|| format!("vertex index out of range"));

                        let p0 = get_v(v0)?;
                        let p1 = get_v(v1)?;
                        let p2 = get_v(v2)?;

                        let mut tri = Triangle::new(
                            [p0.x(), p0.y(), p0.z()],
                            [p1.x(), p1.y(), p1.z()],
                            [p2.x(), p2.y(), p2.z()]
                        );

                        match (n0, n1, n2) {
                            (Some(n0), Some(n1), Some(n2)) => {
                                let get_n = |ni: usize|
                                    normals
                                        .get(ni)
                                        .ok_or_else(|| format!("normal index out of range"));
                                let n0 = get_n(n0)?;
                                let n1 = get_n(n1)?;
                                let n2 = get_n(n2)?;
                                tri.vertex_normals = Matrix::<3, 4>::from_data([
                                    [n0.x(), n0.y(), n0.z(), 0.0],
                                    [n1.x(), n1.y(), n1.z(), 0.0],
                                    [n2.x(), n2.y(), n2.z(), 0.0],
                                ]);
                            }
                            _ => tri.use_computed_normals(),
                        }

                        tris.push(tri);
                    }
                }
                _ => {
                    // Unable to parse token yet Ignore for now
                }
            }
        }

        Ok(Mesh {
            tris: tris,
        })
    }

    fn parse_stl(stl: &[u8]) -> Result<Self, String> {
        // First check if the stl data is binary or ascii by checking if the first few bytes spell out solid or not.
        if Self::is_binary_stl(stl) {
            Self::parse_stl_binary(stl)
        } else {
            let text = std::str
                ::from_utf8(stl)
                .map_err(|_| "STL is not valid ASCII text".to_string())?;
            Self::parse_stl_ascii(text)
        }
    }

    fn parse_stl_ascii(stl: &str) -> Result<Self, String> {
        let mut tris = Vec::new();
        let mut current_verts = Vec::new();
        let mut current_normal: Option<[f32; 3]> = None;

        for line in stl.lines() {
            let line = line.trim();

            if let Some(facet_normal) = line.strip_prefix("facet normal") {
                let mut it = facet_normal.split_whitespace();
                let nx: f32 = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .ok_or("Bad Normal")?;
                let ny: f32 = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .ok_or("Bad Normal")?;
                let nz: f32 = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .ok_or("Bad Normal")?;
                // Ensure the normal isn't a 0-normal
                if
                    !(
                        nx.abs() <= f32::EPSILON &&
                        ny.abs() <= f32::EPSILON &&
                        nz.abs() <= f32::EPSILON
                    )
                {
                    current_normal = Some([nx, ny, nz]);
                }
            } else if let Some(vertex) = line.strip_prefix("vertex") {
                let mut it = vertex.split_whitespace();
                let x: f32 = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .ok_or("Bad Vertex")?;
                let y: f32 = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .ok_or("Bad Vertex")?;
                let z: f32 = it
                    .next()
                    .and_then(|s| s.parse().ok())
                    .ok_or("Bad Vertex")?;
                current_verts.push([x, y, z]);
                if current_verts.len() == 3 {
                    let mut tri = Triangle::new(
                        current_verts[0],
                        current_verts[1],
                        current_verts[2]
                    );
                    match current_normal {
                        Some([nx, ny, nz]) => {
                            tri.vertex_normals = Matrix::<3, 4>::from_data([
                                [nx, ny, nz, 0.0],
                                [nx, ny, nz, 0.0],
                                [nx, ny, nz, 0.0],
                            ]);
                        }
                        None => tri.use_computed_normals(),
                    }
                    tris.push(tri);
                    current_verts.clear();
                    current_normal = None;
                }
            }
        }

        Ok(Self::new(tris))
    }

    fn parse_stl_binary(stl: &[u8]) -> Result<Self, String> {
        //
        if stl.len() < 84 {
            return Err(format!("Invalid Length of File"));
        }

        let triangle_count = u32::from_le_bytes(stl[80..84].try_into().unwrap()) as usize;

        // 84 Bytes for header and 50 Bytes per triangle
        let expected_length = 84 + triangle_count * 50;
        if stl.len() < expected_length {
            return Err(format!("Invalid Length of File"));
        }

        let mut tris = Vec::with_capacity(triangle_count);
        let mut offset = 84_usize;

        for _ in 0..triangle_count {
            tris.push(Self::parse_stl_binary_triangle(stl, offset)?);
            offset += 50;
        }

        return Ok(Self::new(tris));
    }

    fn parse_stl_binary_triangle(stl: &[u8], o: usize) -> Result<Triangle, String> {
        let read_f32 = |o_n: usize| f32::from_le_bytes(stl[o_n..o_n + 4].try_into().unwrap());
        let read_vertex = |o_v: usize| [read_f32(o_v), read_f32(o_v + 4), read_f32(o_v + 8)];

        let v1 = read_vertex(o + 12);
        let v2 = read_vertex(o + 24);
        let v3 = read_vertex(o + 36);

        let mut tri = Triangle::new(v1, v2, v3);
        tri.use_computed_normals();
        return Ok(tri);
    }

    fn is_binary_stl(stl: &[u8]) -> bool {
        if stl.len() < 84 {
            return false;
        }

        let triangle_count = u32::from_le_bytes(stl[80..84].try_into().unwrap()) as usize;
        let expected_len = 84 + triangle_count * 50;

        stl.len() == expected_len
    }

    pub fn construct_cube() -> Self {
        let v: [[f32; 3]; 8] = [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ];

        let faces: [[usize; 3]; 12] = [
            [0, 2, 1],
            [0, 3, 2],
            [4, 5, 6],
            [4, 6, 7],
            [0, 4, 7],
            [0, 7, 3],
            [1, 2, 6],
            [1, 6, 5],
            [0, 1, 5],
            [0, 5, 4],
            [3, 7, 6],
            [3, 6, 2],
        ];

        let tris = faces
            .iter()
            .map(|&[a, b, c]| Triangle::new(v[a], v[b], v[c]))
            .collect();

        Self { tris }
    }

    pub fn transform(&self, transform: Transform) -> Self {
        let tris = self.tris
            .iter()
            .map(|t| t.transform(transform))
            .collect();
        Self { tris }
    }

    pub fn transform_geometry(&self, transform: Transform) -> Self {
        let tris = self.tris
            .iter()
            .map(|t| t.transform_geometry(transform))
            .collect();
        Self { tris }
    }

    pub fn transform_normals(&self, transform: Transform) -> Self {
        let tris = self.tris
            .iter()
            .map(|t| t.transform_normals(transform))
            .collect();
        Self { tris }
    }

    pub fn transform_mut(&mut self, transform: Transform) {
        for tri in self.tris.iter_mut() {
            tri.transform_mut(transform);
        }
    }

    pub fn transform_geometry_mut(&mut self, transform: Transform) {
        for tri in self.tris.iter_mut() {
            tri.transform_geometry_mut(transform);
        }
    }

    pub fn transform_normals_mut(&mut self, transform: Transform) {
        for tri in self.tris.iter_mut() {
            tri.transform_normals_mut(transform);
        }
    }

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.tris
    }

    pub fn use_flat_shading(&mut self) {
        for tri in self.tris.iter_mut() {
            tri.use_computed_normals();
        }
    }

    pub fn mean_triangles_positions(&self) -> RowMat<3> {
        let mut acc = RowMat::<3>::new();
        if self.tris.len() == 0 {
            return acc;
        }
        for tri in self.triangles() {
            acc += tri.center();
        }
        return acc / (self.tris.len() as f32);
    }

    pub fn max_vertex_radius(&self, from: RowMat<3>) -> f32 {
        let mut max_dist_sq = 0.0f32;

        for tri in &self.tris {
            for i in 0..3 {
                let v = tri.verts.row_mat(i).to_uniform();
                let d = v - from;
                let dist_sq = d.x() * d.x() + d.y() * d.y() + d.z() * d.z();
                if dist_sq > max_dist_sq {
                    max_dist_sq = dist_sq;
                }
            }
        }

        max_dist_sq.sqrt()
    }
}
