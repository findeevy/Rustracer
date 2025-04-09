mod definitions;
use definitions::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Model{
  pub verts: Vector3,
  pub faces: Vector3,
}

impl Model{
  pub fn new(filename: &str) -> Self {
    let mut verts = Vec::new();
    let mut faces = Vec::new();
    let path = Path::new(filename);
    let file = match File::open(path) {
      Ok(file) => BufReader::new(file),
      Err(err) => {
        eprintln!("Failed to open {}: {}", filename, err);
        return Model { verts, faces };
      }
    };

    for line in file.lines(){
      let mut parts = line.split_whitespace();
      if let Some(prefix) = parts.next(){
        match prefix {
          //Vertices.
          "v" => {
            let mut v = Vector3::new(0.0, 0.0, 0.0);
            for i in 0..3{
              if let Some(val) = parts.next(){
                if let Ok(f) = val.parse::<f32>() {
                  *v.get_mut(i) = f;
                }
              }
            }
            verts.push(v);
          }
          
          //Faces.
          "f" => {
            let mut f = Vector3::new(0.0, 0.0, 0.0);
            let mut count = 0;
            while let Some(idx_str) = parts.next(){
              if let Ok(idk) = idx_str.parse::<i32>(){
                if count < 3 {
                  match count {
                    0 => f.x = idx - 1,
                    1 => f.y = idx - 1,
                    2 => f.z = idx - 1,
                    _ => {}
                  }
                }
              }
              count += 1;
            }
            if count == 3 {
              faces.push(f);
            }
          }

          //Other data.
          _ => {
            //Do nothing.
          }

        }
      }
    
    }
  }
}
 
