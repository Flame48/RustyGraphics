use crate::scene::math::matrix::{ Quaternion, RowMat, SqMat };

// MARK: Invertible Transforms
#[derive(Clone, Copy)]
pub struct Transform {
    pub forward: SqMat<4>,
    pub reverse: SqMat<4>,
}

impl Default for Transform {
    fn default() -> Self {
        Self { forward: SqMat::<4>::identity(), reverse: SqMat::<4>::identity() }
    }
}

impl Transform {
    pub fn extend_forward(&self, by: Self) -> Self {
        Self {
            forward: self.forward * by.forward,
            reverse: by.reverse * self.reverse,
        }
    }
    pub fn extend_forward_mut(&mut self, by: Self) {
        *self = self.extend_forward(by);
    }

    pub fn extend_reverse(&self, by: Self) -> Self {
        Self {
            forward: by.forward * self.forward,
            reverse: self.reverse * by.reverse,
        }
    }

    pub fn extend_reverse_mut(&mut self, by: Self) {
        *self = self.extend_reverse(by);
    }

    pub fn scale(s: RowMat<3>) -> Self {
        Transform {
            forward: SqMat::<4>::scale(s),
            reverse: SqMat::<4>::scale_inv(s),
        }
    }
    pub fn translation(p: RowMat<3>) -> Self {
        Transform {
            forward: SqMat::<4>::translation(p),
            reverse: SqMat::<4>::translation_inv(p),
        }
    }

    pub fn rotation(q: Quaternion) -> Self {
        Transform {
            forward: SqMat::<4>::rotation(q),
            reverse: SqMat::<4>::rotation_inv(q),
        }
    }

    pub fn inverse(&self) -> Self {
        Self { forward: self.reverse, reverse: self.forward }
    }
}
