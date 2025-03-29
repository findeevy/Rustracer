#![allow(warnings)]

use std::ops::{Add, Sub, Mul};
use std::fs::File;
use std::io::{self, Write};

const HEIGHT: usize = 1024;
const WIDTH: usize = 1024;

#[derive(Debug, Copy, Clone)]
struct Material{
  diffuse_color: Vector3,
}

impl Material{

  fn new(diffuse_color: Vector3) -> Self {
    Material {diffuse_color}
  }
  
}

#[derive(Debug, Copy, Clone)]
struct Sphere{
  transform: Vector3,
  radius: f32,
  material: Material,
}

impl Sphere{

  fn new(transform: Vector3, radius: f32, material: Material) -> Self {
    Sphere {transform, radius, material}
  }
  
}

#[derive(Debug, Copy, Clone)]
struct Vector3{
  x: f32,
  y: f32,
  z: f32,
}

//Vector3 functionality.
impl Vector3{
  //Initiate a new Vector3 of three floats.
  fn new(x:f32, y:f32, z:f32) -> Self {
    Vector3 {x, y, z}
  }

  fn tou8(self) -> [u8; 3] {
      [self.x as u8, self.y as u8, self.z as u8]
  }

  //Compute the dot product.
  fn dot(&self, other: &Vector3) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
  
  //Computes the length of the vector.
  fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }
  
  //Normalizes the vector.
  fn normalize(&self) -> Vector3 {
    let magnitude = self.magnitude();
    if(magnitude == 0.0){
      return Vector3 {x: 0.0, y: 0.0, z:0.0}
    }

    Vector3{
      x: self.x/magnitude,
      y: self.y/magnitude,
      z: self.z/magnitude,
    }
  }
  

}

//Vector3 addition functionality.
impl Add<Vector3> for Vector3{
  type Output = Vector3;

  fn add(self, other: Vector3) -> Vector3{
    Vector3{
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

//Vector3 subtraction functionality.
impl Sub<Vector3> for Vector3{
  type Output = Vector3;
  
  fn sub(self, other: Vector3) -> Vector3{
    Vector3{
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}

//Vector3 multiplication functionality.
impl Mul<f32> for Vector3{
  type Output = Vector3;
  
  fn mul(self, scalar: f32) -> Vector3{
    Vector3 {
      x: self.x*scalar,
      y: self.y*scalar,
      z: self.z*scalar,
    }
  }

}

//Divide two usizes and return a float.
fn udiv(x: usize, y: usize) -> f32{
    return (x as f32)/(y as f32);
}

fn ray_intersect(sphere: Sphere, origin: Vector3, direction: Vector3, distance: f32) -> bool{
  let length = sphere.transform - origin;
  let ray = length.dot(&direction);
  //println!("{}", ray);
  let difference_of_squares = length.dot(&length) - ray*ray;
  //println!("{}", difference_of_squares);
  if (difference_of_squares > sphere.radius){
    return false;
  }
  let temp = (sphere.radius*sphere.radius - difference_of_squares).sqrt();
  let mut point0 = ray - temp;
  let point1 = ray + temp;
  if (point0 < 0.0){
    point0 = point1;
  }
  return !(point0 < 0.0);
}

fn scene_intersect<'a>(origin: Vector3, direction: Vector3, spheres: &Vec<Sphere>, mut hit: Vector3, mut N: Vector3, mut material: Material) -> Option<(Vector3, Vector3, Material)>{
  let mut spheres_distance = f32::MAX;  
  for i in 0..spheres.len() {
    let mut dist_i: f32 = 0.0;
    if (ray_intersect(spheres[i], origin, direction, dist_i) && dist_i < spheres_distance){
      spheres_distance = dist_i;
      hit = origin + direction*dist_i;
      N = (hit-spheres[i].transform).normalize();
      material = spheres[i].material;
      println!("{:?}", material);
    }
  }
  if (spheres_distance < 1000.0){
    return Some((hit, N, material));
  }
  return None;
}



//Write the framebuffer to a ppm file.
fn framebuffer_to_ppm(width: usize, height: usize, framebuffer: &Vec<Vector3>) -> io::Result<()>{
    //Open the PPM file.
    let mut file = File::create("./out.ppm")?;

    //Write the header for the PPM file (P6 format, width, height, max color value).
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    //Write the pixel data.
    for y in 0..height {
        for x in 0..width {
            //Scale between 0 to 255 and convert to u8.
            let pixel_value = Vector3::new(framebuffer[x+y*WIDTH].x, framebuffer[x+y*WIDTH].y, framebuffer[x+y*WIDTH].z);
            let clamped = Vector3::tou8(pixel_value * 255.0);
            for i in clamped{
              file.write_all(&[i])?;
            }
        }
    }

    Ok(())
}


fn cast_ray(origin: Vector3, direction: Vector3, spheres: &Vec<Sphere>) -> Vector3{
  let mut N: Vector3 = Vector3::new(0.0, 0.0, 0.0);
  let mut point: Vector3 = Vector3::new(0.0, 0.0, 0.0);
  let mut material: Material = Material::new(Vector3::new(0.0, 0.0, 0.0));
  if let Some((_, _, material)) = scene_intersect(origin, direction, &spheres, point, N, material) {
    return material.diffuse_color;
  }
  return Vector3::new(0.3, 0.3, 0.9);
}

fn render_test_gradient(){
  let mut framebuffer: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
  
  //Create a gradient of pixels.
  for y in 0..HEIGHT{
    for x in 0..WIDTH{
      framebuffer[x+y*WIDTH] = Vector3::new(udiv(y, HEIGHT), 0.0, udiv(x, WIDTH));
    }
  }

  //Write our gradient to a PPM.
  let _ = framebuffer_to_ppm(WIDTH, HEIGHT, &framebuffer);
}

fn render(spheres: &Vec<Sphere>){
  let mut framebuffer: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
  let fov: f32 = 1.0;
  for y in 0..HEIGHT{
    for x in 0..WIDTH{
      let transform_x = (2.0*(x as f32 + 0.5)/(WIDTH as f32) - 1.0)*(fov/2.0).tan()*udiv(WIDTH, HEIGHT);
      let transform_y = -1.0*(2.0*(y as f32 + 0.5)/(HEIGHT as f32) - 1.0)*(fov/2.0).tan();
      let direction = Vector3::new(transform_x, transform_y, -1.0).normalize();
      framebuffer[x+y*WIDTH] = cast_ray(Vector3::new(0.0, 0.0, 0.0), direction, &spheres);
    }
  }

  let _ = framebuffer_to_ppm(WIDTH, HEIGHT, &framebuffer);
}

fn main(){
  let red = Material::new(Vector3::new(1.0, 0.0, 0.0));
  let blue = Material::new(Vector3::new(0.0, 1.0, 0.0));
  let green = Material::new(Vector3::new(0.0, 0.0, 1.0));

  let mut spheres: Vec<Sphere> = Vec::new();
  spheres.push(Sphere::new(Vector3::new(-3.0, 0.0, -16.0), 1.0, red));
  spheres.push(Sphere::new(Vector3::new(-1.0, 1.0, -16.0), 1.5, blue));
  
  render(&spheres)
}
