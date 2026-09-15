use std::sync::Arc;

use crate::scene::{ math::matrix::RowMat, renderer::RGBA };

pub trait SamplerTarget {
    fn sample(&self, u: f32, v: f32) -> RGBA;

    fn sample_uv(&self, uv: RowMat<2>) -> RGBA {
        self.sample(uv.get(0, 0), uv.get(0, 1))
    }
}

pub struct ConstantSamplerTarget {
    value: RGBA,
}

impl SamplerTarget for ConstantSamplerTarget {
    fn sample(&self, _u: f32, _v: f32) -> RGBA {
        self.value
    }
}

impl ConstantSamplerTarget {
    pub fn new(value: RGBA) -> Self {
        Self { value }
    }
}

#[derive(Clone)]
pub struct Sampler {
    pub target: Arc<dyn SamplerTarget>,
}

impl SamplerTarget for Sampler {
    fn sample(&self, u: f32, v: f32) -> RGBA {
        self.target.sample(u, v)
    }
}

impl Sampler {
    pub fn new<T: 'static>(target: T) -> Self where T: SamplerTarget {
        Self { target: Arc::new(target) }
    }

    pub fn maybe<T: 'static>(target: Option<T>) -> Option<Self> where T: SamplerTarget {
        Some(Self::new(target?))
    }

    pub fn value(v: RGBA) -> Self {
        Self { target: Arc::new(ConstantSamplerTarget::new(v)) }
    }
}
