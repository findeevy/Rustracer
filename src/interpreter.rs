use std::collections::HashMap;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufRead;

//Import all of our structs from definitions.rs
use definitions::Vector4;
use definitions::Vector3;
use definitions::Light;
use definitions::Sphere;
use definitions::Material;

//Import our model struct from model.rs
use model::Model;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn interpreter(input: String) -> (Vec<Light>, Vec<Sphere>, Vec<Model>, Vector3, i32, usize, usize, i32){
  let mut materials: HashMap<String, Material> = HashMap::new();
  let mut lights: Vec<Light> = Vec::new();
  let mut spheres: Vec<Sphere> = Vec::new();
  let mut meshes: Vec<Model> = Vec::new();
  let mut background_color: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};
  let mut path_depth: i32 = 5;
  let mut fheight: usize = 1280;
  let mut fwidth: usize = 720;
  let mut anti_alias: i32 = 1;
  if let Ok(lines) = read_lines(input) {
    for line in lines.map_while(Result::ok) {
      let line_split: Vec<&str> = line.split_whitespace().collect();
      //This is super messy, I'll look into making it way cleaner soon...
      match line.chars().take(2).collect::<String>().as_str() {
        "h " => fheight = line_split[1].parse().unwrap(),
        "w " => fwidth = line_split[1].parse().unwrap(),
        "r " => path_depth = line_split[1].parse().unwrap(),
        "aa" => anti_alias = line_split[1].parse().unwrap(),
        "bg" => background_color = Vector3{x: line_split[1].parse().unwrap(), y: line_split[2].parse().unwrap(), z: line_split[3].parse().unwrap()},
        "mt" => {materials.insert(line_split[1].to_string(), Material::new(Vector3::new(line_split[2].parse().unwrap(), line_split[3].parse().unwrap(), line_split[4].parse().unwrap()), Vector4::new(line_split[5].parse().unwrap(), line_split[6].parse().unwrap(), line_split[7].parse().unwrap(), line_split[8].parse().unwrap()), line_split[9].parse().unwrap(), line_split[10].parse().unwrap()));},
        "l " => lights.push(Light::new(Vector3::new(line_split[1].parse().unwrap(), line_split[2].parse().unwrap(), line_split[3].parse().unwrap()), line_split[4].parse().unwrap())),
        "sp" => spheres.push(Sphere::new(Vector3::new(line_split[1].parse().unwrap(), line_split[2].parse().unwrap(), line_split[3].parse().unwrap()), line_split[4].parse().unwrap(), materials[line_split[5]])),
        "ms" => meshes.push(Model::new(line_split[1], Vector3::new(line_split[2].parse().unwrap(), line_split[3].parse().unwrap(), line_split[4].parse().unwrap()), materials[line_split[5]])),
        _ => (),
      }
    }
  }
  return (lights, spheres, meshes, background_color, path_depth, fheight, fwidth, anti_alias);
}