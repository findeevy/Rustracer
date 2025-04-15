use std::ops::{Add, Sub, Mul};
#[derive(Debug, Copy, Clone)]
pub struct Light{
  pub intensity: f32,
  pub transform: Vector3,
}

impl Light{

  pub fn new(transform: Vector3, intensity: f32) -> Self {
    Light {transform, intensity}
  }
  
}

#[derive(Debug, Copy, Clone)]
pub struct Material{
  pub diffuse_color: Vector3,
  pub albedo: Vector4,
  pub specular_exponent: f32,
  pub refractive_index: f32,
}

impl Material{

  pub fn new(diffuse_color: Vector3, albedo: Vector4, specular_exponent: f32, refractive_index: f32) -> Self {
    Material {diffuse_color, albedo, specular_exponent, refractive_index}
  }
  
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere{
  pub transform: Vector3,
  pub radius: f32,
  pub material: Material,
}

impl Sphere{

  pub fn new(transform: Vector3, radius: f32, material: Material) -> Self {
    Sphere {transform, radius, material}
  }
  
}

#[derive(Debug, Copy, Clone)]
pub struct Vector2{
  pub x: f32,
  pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3i{
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl Vector3i{
  pub fn new(x:i32, y:i32, z:i32) -> Self{
    Vector3i {x, y, z}
  }
}


#[derive(Debug, Copy, Clone)]
pub struct Vector3{
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

//Vector3 functionality.
impl Vector3{
  //Initiate a new Vector3 of three floats.
  pub fn new(x:f32, y:f32, z:f32) -> Self {
    Vector3 {x, y, z}
  }

  pub fn tou8(self) -> [u8; 3] {
      [self.x as u8, self.y as u8, self.z as u8]
  }

  //Compute the dot product.
  pub fn dot(&self, other: &Vector3) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  //Compute the cross product.
  pub fn cross(&self, other: &Vector3) -> Vector3{
    Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }


  //Computes the length of the vector.
  pub fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }
  
  //Normalizes the vector.
  pub fn normalize(&self) -> Vector3 {
    let magnitude = self.magnitude();
    if magnitude == 0.0 {
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

#[derive(Debug, Copy, Clone)]
pub struct Vector4{
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub a: f32,
}

//Vector32functionality.
impl Vector4{
  //Initiate a new Vector2 of two floats.
  pub fn new(x:f32, y:f32, z:f32, a:f32) -> Self {
    Vector4 {x, y, z, a}
  }
}
