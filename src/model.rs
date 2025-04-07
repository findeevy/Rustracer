
mod definitions;
use definitions::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Model{
  pub verts: Vector3,
  pub faces: Vector3,
}

impl Model{
  pub fn new(filename: &str) -> Self {
    Model {verts, faces}
  }
}
 