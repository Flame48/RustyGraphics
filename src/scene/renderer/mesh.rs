use std::{ fs, path::Path, str::FromStr };

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
        let mut verts = self.verts * transform.forward;
        verts.normalize_homogenous_mut();
        let mut vertex_normals = self.vertex_normals * transform.forward;
        for i in 0..3 {
            let n = vertex_normals.row_mat(i).to_uniform().norm_row();
            vertex_normals.data[i] = [n.x(), n.y(), n.z(), 0.0];
        }
        Self { verts, vertex_normals, vertex_data: [VertexData::new(); 3] }
    }

    fn compute_face_normal(&self) -> RowMat<4> {
        let v0 = self.verts.row_mat(0).to_uniform();
        let v1 = self.verts.row_mat(1).to_uniform();
        let v2 = self.verts.row_mat(2).to_uniform();
        (v2 - v0)
            .cross(v1 - v0)
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
}

pub struct Mesh {
    tris: Vec<Triangle>,
}

impl Mesh {
    pub fn import_obj(path: impl AsRef<Path>) -> Result<Self, String> {
        let Ok(contents) = fs::read_to_string(path) else {
            return Err(format!("Unable to open file"));
        };
        Self::parse_obj(&contents)
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

    fn parse_face_vertex_indices(from: &str) -> (Option<usize>, Option<usize>, Option<usize>) {
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

    pub fn parse_obj(obj: &str) -> Result<Self, String> {
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
                            Self::parse_face_vertex_indices(vertex_info_s) else {
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

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.tris
    }

    pub fn use_flat_shading(&mut self) {
        for tri in self.tris.iter_mut() {
            tri.use_computed_normals();
        }
    }
}
