#![allow(warnings)]
mod definitions;
use definitions::Vector3;
use definitions::Vector2;
use definitions::Light;
use definitions::Sphere;
use definitions::Material;

use std::ops::{Add, Sub, Mul};
use std::fs::File;
use std::io::{self, Write};
use std::process;

const HEIGHT: usize = 768;
const WIDTH: usize = 1024;

//Divide two usizes and return a float.
fn udiv(x: usize, y: usize) -> f32{
    return (x as f32)/(y as f32);
}

fn reflect(I: Vector3, N: Vector3) -> Vector3{
  return I - N*2.0*(I.dot(&N));
}

fn ray_intersect(sphere: Sphere, origin: Vector3, direction: Vector3, mut distance: f32) -> Option<f32>{
  let length = sphere.transform - origin;
  let ray = length.dot(&direction);
  //println!("{}", ray);
  let difference_of_squares = (length.dot(&length)) - ray*ray;
  //println!("{}", difference_of_squares);
  if (difference_of_squares > sphere.radius*sphere.radius){
    return None;
  }
  let temp = (sphere.radius*sphere.radius - difference_of_squares).sqrt();
  distance = ray - temp;
  let point1 = ray + temp;
  if (distance < 0.0){
    distance = point1;
  }
  if !(distance < 0.0){
    return Some((distance));
  }
  else{
    return None;
  }
}

fn scene_intersect<'a>(origin: Vector3, direction: Vector3, spheres: &Vec<Sphere>, mut hit: Vector3, mut N: Vector3, mut material: Material) -> Option<(Vector3, Vector3, Material)>{
  let mut spheres_distance = f32::MAX;  
  for i in 0..spheres.len() {
    let mut dist_i: f32 = 0.0;
    if let Some((dist_i)) = ray_intersect(spheres[i], origin, direction, dist_i){
      if dist_i < spheres_distance {
        spheres_distance = dist_i;
        hit = origin + direction*dist_i;
        //println!("{:?}", hit);
        //process::exit(0);
        N = (hit-spheres[i].transform).normalize();
        material = spheres[i].material;
      }
    }
  }
  if (spheres_distance < 1000.0){
    return Some((hit, N, material));
  }
  return None;
}



//Write the framebuffer to a ppm file.
fn framebuffer_to_ppm(width: usize, height: usize, framebuffer: &mut Vec<Vector3>) -> io::Result<()>{
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
            let mut c = framebuffer[x+y*WIDTH];
            let max = f32::max(c.x, f32::max(c.y, c.z)); 
        
            if max > 1.0 {
                let scale_factor = 1.0 / max;
                c = c * scale_factor;
            } 
            framebuffer[x+y*WIDTH] = c;

            let pixel_value = Vector3::new(framebuffer[x+y*WIDTH].x, framebuffer[x+y*WIDTH].y, framebuffer[x+y*WIDTH].z);
            let clamped = Vector3::tou8(pixel_value * 255.0);
            for i in clamped{
              file.write_all(&[i])?;
            }
        }
    }

    Ok(())
}


fn cast_ray(origin: Vector3, direction: Vector3, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vector3{
  let mut N: Vector3 = Vector3::new(0.0, 0.0, 0.0);
  let mut point: Vector3 = Vector3::new(0.0, 0.0, 0.0);
  let mut material: Material = Material::new(Vector3::new(0.0, 0.0, 0.0), Vector2::new(0.0, 0.0), 0.0);
  let mut diffuse_light_intensity: f32 = 0.0;
  let mut specular_light_intensity: f32 = 0.0;
  if let Some((point, N, material)) = scene_intersect(origin, direction, &spheres, point, N, material) {
    for i in 0..lights.len(){
      let light_direction: Vector3 = (lights[i].transform - point).normalize();
      let light_distance = (lights[i].transform - point).magnitude();
      //Checking for shadows here.
      let mut shadow_origin: Vector3 = Vector3::new(0.0, 0.0, 0.0);
      if (light_direction.dot(&N) < 0.0){
        shadow_origin = point + (N * 0.001);
      }
      else{
        shadow_origin = point - (N * 0.001);
      }
      let shadow_pt: Vector3 = Vector3::new(0.0, 0.0, 0.0);
      let shadow_N: Vector3 = Vector3::new(0.0, 0.0, 0.0);
      let temp_material: Material = Material::new(Vector3::new(0.0, 0.0, 0.0), Vector2::new(1.0, 0.1), 0.0);
      //process::exit(0)
      if let Some((shadow_pt, shadow_N, temp_material)) = scene_intersect(shadow_origin, light_direction, &spheres, shadow_pt, shadow_N, temp_material){
        if ((shadow_pt-shadow_origin).magnitude() < light_distance){
          println!("{:?}", shadow_pt);
          process::exit(0);
        }
      }
      println!("OOP");
      diffuse_light_intensity += lights[i].intensity * light_direction.dot(&N).max(0.0);
      specular_light_intensity += (f32::max(0.0, (reflect(light_direction * -1.0, N)* -1.0).dot(&direction))).powf(material.specular_exponent) * lights[i].intensity;
    }
    return (material.diffuse_color * diffuse_light_intensity * material.albedo.x) + ((Vector3::new(1.0, 1.0, 1.0)) * specular_light_intensity * material.albedo.y);
  }
  return Vector3::new(0.1, 0.1, 0.7);
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
  let _ = framebuffer_to_ppm(WIDTH, HEIGHT, &mut framebuffer);
}

fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>){
  let mut framebuffer: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
  let fov: f32 = 1.0;
  for y in 0..HEIGHT{
    for x in 0..WIDTH{
      let transform_x = (2.0*(x as f32 + 0.5)/(WIDTH as f32) - 1.0)*(fov/2.0).tan()*udiv(WIDTH, HEIGHT);
      let transform_y = -1.0*(2.0*(y as f32 + 0.5)/(HEIGHT as f32) - 1.0)*(fov/2.0).tan();
      let direction = Vector3::new(transform_x, transform_y, -1.0).normalize();
      framebuffer[x+y*WIDTH] = cast_ray(Vector3::new(0.0, 0.0, 0.0), direction, &spheres, &lights);
    }
  }

  let _ = framebuffer_to_ppm(WIDTH, HEIGHT, &mut framebuffer);
}

fn main(){
  let shiny = Material::new(Vector3::new(0.4, 0.4, 0.4), Vector2::new(0.6, 0.3), 80.0);
  let dull = Material::new(Vector3::new(0.1, 0.3, 0.1), Vector2::new(0.9, 0.1), 10.0);
  
  let mut lights: Vec<Light> = Vec::new();
  lights.push(Light::new(Vector3::new(-20.0, 20.0, 20.0), 1.5));
  lights.push(Light::new(Vector3::new(30.0, 50.0, -25.0), 1.8));
  lights.push(Light::new(Vector3::new(30.0, 20.0, 30.0), 1.7));

  let mut spheres: Vec<Sphere> = Vec::new();
  spheres.push(Sphere::new(Vector3::new(-3.0, 0.0, -16.0), 2.0, dull));
  spheres.push(Sphere::new(Vector3::new(-1.0, -1.5, -12.0), 2.0, shiny));
  spheres.push(Sphere::new(Vector3::new(1.5, -0.5, -18.0), 3.0, shiny));
  spheres.push(Sphere::new(Vector3::new(7.0, 5.0, -18.0), 4.0, dull));
  
  render(&spheres, &lights)
}
