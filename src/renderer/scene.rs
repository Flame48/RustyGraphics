use crate::renderer::math::matrix::{ Matrix, RowMat, ColMat, SqMat };

/// Node's keep track of their position / rotation / scale within the scene tree
pub trait Node {
    fn position(&self) -> RowMat<3>;
    fn rotation(&self) -> RowMat<4>;
    fn scale(&self) -> RowMat<3>;

    /// Matrix for applying transforms to the vertices, converting from node's local space to parent space.
    fn matrix(&self) -> SqMat<4> {
        let s = SqMat::<4>::scale(self.scale().serial_row());
        let r = SqMat::<4>::rotation(self.rotation().serial_row());
        let t = SqMat::<4>::translation(self.position().serial_row());
        s * r * t
    }
}

struct Camera {}

struct Scene {}
