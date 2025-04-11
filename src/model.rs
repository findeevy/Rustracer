use definitions::Vector3;
use definitions::Vector3i;
use definitions::Material;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::path::Path;
use std::str::SplitWhitespace;

#[derive(Debug, Clone)]
pub struct Model{
  pub verts: Vec<Vector3>,
  pub faces: Vec<Vector3i>,
  pub transform: Vector3,
  pub material: Material,
}

impl Model{
  pub fn new(filename: &str, transform: Vector3, material: Material) -> Self {
    let mut verts: Vec<Vector3> = Vec::new();
    let mut faces: Vec<Vector3i> = Vec::new();
    let path = Path::new(filename);
    let file = match File::open(path) {
      Ok(file) => BufReader::new(file),
      Err(err) => {
        eprintln!("Failed to open {}: {}", filename, err);
        return Model {verts, faces, transform, material};
      }
    };
    let mut reader = BufReader::new(file);

  for line in reader.lines() {
      let line = line.unwrap();
      let parts: Vec<&str> = line.trim().split_whitespace().collect();
      if parts.is_empty() { continue; }

      match parts[0] {
          "v" => {
              let x: f32 = parts[1].parse().unwrap();
              let y: f32 = parts[2].parse().unwrap();
              let z: f32 = parts[3].parse().unwrap();
              verts.push(Vector3::new(x, y, z));
          },
          "f" => {
              let mut indices = Vec::new();
              for part in &parts[1..] {
                  let index_part = part.split('/').next().unwrap();
                  let index = index_part.parse::<i32>().unwrap() - 1;
                  indices.push(index);
              }
              if indices.len() >= 3 {
                  for i in 2..indices.len() {
                      faces.push(Vector3i::new(indices[0], indices[i-1], indices[i]));
                  }
              }
          },
          _ => {}
      }
    }
    return Model {verts, faces, transform, material};
  }

}
 
