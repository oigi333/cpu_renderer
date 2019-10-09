use crate::vector::{Vector2, Vector3};

pub trait Texture {
    fn sample(&self, uv: Vector2) -> Vector3;
}