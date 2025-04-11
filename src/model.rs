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
        return Model { verts, faces };
      }
    };

    for line_result in file.lines(){
      if let Ok(line) = line_result {
        let mut parts = line.split_whitespace();
        if let Some(prefix) = parts.next(){
          match prefix {
            //Vertices.
            "v" => {
              let mut v = Vector3::new(0.0, 0.0, 0.0);
              for i in 0..3{
                if let Some(val) = parts.next(){
                  if let Ok(f) = val.parse::<f32>() {
                    match i {
                      0 => v.x = f,
                      1 => v.y = f,
                      2 => v.z = f,
                      _ => {}
                    }
                  }
                }
              }
              verts.push(v);
            }  
             //Faces.
            "f" => {
              let mut f = Vector3i::new(0, 0, 0);
              if let Some(val) = parts.next(){
                let temp = val.split("/").collect::<Vec<_>>();
                let mut i = 0;
                for part in temp{
                  if let Ok(v) = part.parse::<i32>() {
                    match i {
                        0 => f.x = v,
                        1 => f.y = v,
                        2 => f.z = v,
                        _ => {}
                    }
                  }
                  i += 1;
                }
              }
              faces.push(f);
            }
            //Other data.
            _ => {
              //Do nothing.
            }
          }
        } 
      }    
    }
    return Model {verts, faces, transform, material};
  }

}
 
