#![allow(warnings)]

mod definitions;
mod model;

//Import our model struct from model.rs
use model::Model;

//Import all of our structs from definitions.rs
use definitions::Vector4;
use definitions::Vector3;
use definitions::Vector2;
use definitions::Light;
use definitions::Sphere;
use definitions::Material;

//Import all of the standard libraries we need.
use std::ops::{Add, Sub, Mul};
use std::fs::File;
use std::io::{self, Write};
use std::process;
use std::mem;
use std::thread;
use std::time::{SystemTime, Instant};
use std::sync::{Arc};
use std::sync::mpsc::sync_channel;

//Constants
const BACKGROUND_COLOR: Vector3 = Vector3{x: 0.3, y: 0.6, z: 0.9};
const PATH_DEPTH: i32 = 4;
const HEIGHT: usize = 100;
const WIDTH: usize = 100;
const FOURX_AA: [Vector2; 4] = [Vector2{x: 0.25, y: 0.25}, Vector2{x: -0.25, y: 0.25}, Vector2{x: 0.25, y: -0.25}, Vector2{x: -0.25, y:-0.25}];

//Divide two usizes and return a float.
fn udiv(x: usize, y: usize) -> f32{
    return (x as f32)/(y as f32);
}

//Refraction (Using Snell's law).
fn refract(I: Vector3, N: Vector3, refractive_index: f32) -> Vector3{
  let mut cosi: f32 = -1.0*(f32::max(-1.0, f32::min(1.0, I.dot(&N))));
  let mut etai: f32 = 1.0;
  let mut etat: f32 = refractive_index;
  let mut n: Vector3 = N;

  if cosi < 0.0{
    cosi = cosi*(-1.0);
    mem::swap(&mut etai, &mut etat);
    n = N*(-1.0);
  }

  let eta: f32 = etai / etat;
  let k: f32 = 1.0 - eta*eta*(1.0-cosi*cosi);

  if k < 0.0{
    return Vector3::new(0.0, 0.0, 0.0);
  }
  return I*eta + n*(eta*cosi-k.sqrt());
}

//Reflection
fn reflect(I: Vector3, N: Vector3) -> Vector3{
  return I - N*2.0*(I.dot(&N));
}

//Checks if a ray hits an object.
fn ray_intersect(sphere: Sphere, origin: Vector3, direction: Vector3, mut distance: f32) -> Option<f32>{
  let length = sphere.transform - origin;
  let ray = length.dot(&direction);
  let difference_of_squares = (length.dot(&length)) - ray*ray;
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

//Runs through list of objects in the scene and checks for intersection.
fn scene_intersect<'a>(origin: Vector3, direction: Vector3, spheres: &Vec<Sphere>, mut hit: Vector3, mut N: Vector3, mut material: Material) -> Option<(Vector3, Vector3, Material)>{
  let mut spheres_distance = f32::MAX;  
  for i in 0..spheres.len() {
    let mut dist_i: f32 = 0.0;
    if let Some((dist_i)) = ray_intersect(spheres[i], origin, direction, dist_i){
      if dist_i < spheres_distance {
        spheres_distance = dist_i;
        hit = origin + direction*dist_i;
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
fn framebuffer_to_ppm(framebuffer: &mut Vec<Vector3>) -> io::Result<()>{
    //Open the PPM file.
    let mut file = File::create("./out.ppm")?;
    let start_time = Instant::now();
    //Write the header for the PPM file (P6 format, width, height, max color value).
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", WIDTH, HEIGHT)?;
    writeln!(file, "255")?;

    //Write the pixel data.
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
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
            print!("\r{:?}% of the image written to disk.", (udiv(x+y*WIDTH, HEIGHT*WIDTH)*100.0 + 1.0) as i32);
        }
    }
    let duration = start_time.elapsed();
    println!("\nWriting completed in {} seconds.", duration.as_secs_f64());
    Ok(())
}

//Raycast function, uses reflection, refraction, and calculates shadows.
fn cast_ray(origin: Vector3, direction: Vector3, spheres: &Vec<Sphere>, lights: &Vec<Light>, depth: i32) -> Vector3{
  let mut N: Vector3 = Vector3::new(0.0, 0.0, 0.0);
  let mut point: Vector3 = Vector3::new(0.0, 0.0, 0.0);
  let mut material: Material = Material::new(Vector3::new(0.0, 0.0, 0.0), Vector4::new(0.0, 0.0, 0.0, 0.0), 0.0, 0.0);
  let mut diffuse_light_intensity: f32 = 0.0;
  let mut specular_light_intensity: f32 = 0.0;

  //Check if we've exceeded the path depth to limit render times.
  if (depth <= PATH_DEPTH){
    if let Some((point, N, material)) = scene_intersect(origin, direction, &spheres, point, N, material) {
      let mut reflect_color: Vector3 = Vector3::new(0.0, 0.0, 0.0);
      let mut refract_color: Vector3 = Vector3::new(0.0, 0.0, 0.0);
      for i in 0..lights.len(){
        let light_direction: Vector3 = (lights[i].transform - point).normalize();
        let light_distance = (lights[i].transform - point).magnitude();
        //Checking for reflection and refraction.
        let reflect_direction: Vector3 = reflect(direction, N).normalize();
        let refract_direction: Vector3 = refract(direction, N, material.refractive_index).normalize();
        let mut reflect_origin: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        let mut refract_origin: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        if (reflect_direction.dot(&N) < 0.0){
          reflect_origin = point - (N * 0.001);
        }
        else{
          reflect_origin = point + (N * 0.001);
        }

        if (refract_direction.dot(&N) < 0.0){
          refract_origin = point - (N * 0.001);
        }
        else{
          refract_origin = point + (N * 0.001);
        }

        reflect_color = cast_ray(reflect_origin, reflect_direction, spheres, lights, depth + 1);
        refract_color = cast_ray(refract_origin, refract_direction, spheres, lights, depth + 1);
        //Checking for shadows here.
        let mut shadow_origin: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        if (light_direction.dot(&N) < 0.0){
          shadow_origin = point - (N * 0.001);
        }
        else{
          shadow_origin = point + (N * 0.001);
        }
        let shadow_pt: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        let shadow_N: Vector3 = Vector3::new(0.0, 0.0, 0.0);
        let temp_material: Material = Material::new(Vector3::new(0.0, 0.0, 0.0), Vector4::new(0.0, 0.0, 0.0, 0.0), 0.0, 0.0);
        if let Some((shadow_pt, shadow_N, temp_material)) = scene_intersect(shadow_origin, light_direction, &spheres, shadow_pt, shadow_N, temp_material){
          if ((shadow_pt-shadow_origin).magnitude() < light_distance){
            continue;
          }
        }
        diffuse_light_intensity += lights[i].intensity * light_direction.dot(&N).max(0.0);
        specular_light_intensity += (f32::max(0.0, (reflect(light_direction * -1.0, N) * -1.0).dot(&direction))).powf(material.specular_exponent) * lights[i].intensity;
      }
      //Compute the final color of the pixel.
      return (material.diffuse_color * diffuse_light_intensity * material.albedo.x) + ((Vector3::new(1.0, 1.0, 1.0)) * specular_light_intensity * material.albedo.y) + reflect_color*material.albedo.z + refract_color*material.albedo.a;
    }
  }
  //If nothing is hit, just return the background color of the render.
  return BACKGROUND_COLOR;
}

//Our main rendering function that takes in our objects and lights.
fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>){
  let mut threads = 1 as usize;
  //Check how many threads we have access to.
  match thread::available_parallelism() {
    Ok(parallelism) => {
      threads = parallelism.into();
      println!("{} threads available.", threads);
    }
    Err(e) => {
      eprintln!("Failed to get available threads: {}", e);
    }
  }
  
  //Initialize all of our data structres for the threads.
  let start_time = Instant::now();
  let mut handles = vec![];
  let (tx, rx) = sync_channel(threads);

  let mut framebuffer: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
  let fov: f32 = 1.0;

  let spheres_arc = Arc::new(spheres.clone());
  let lights_arc = Arc::new(lights.clone());

  //Create a chunk of the render for each thread to compute.
  for j in 0..threads{
    let start_y = ((j as f32)*udiv(HEIGHT, threads)) as usize;
    let end_y = ((j as f32 + 1.0)*udiv(HEIGHT, threads)) as usize;
    let tx = tx.clone();
    let spheres_clone = Arc::clone(&spheres_arc);
    let lights_clone = Arc::clone(&lights_arc);

    let handle = thread::spawn(move || {
    //Iterate through each pixel in the chunk and render it via ray-tracing.
    for y in start_y..end_y{
      for x in 0..WIDTH{
        let mut color = Vector3::new(0.0, 0.0, 0.0);
        //Caat four rays for anti-aliasing.
        for i in 0..FOURX_AA.len(){
          let transform_x = (2.0*(x as f32 + 0.5 + FOURX_AA[i].x)/(WIDTH as f32) - 1.0)*(fov/2.0).tan()*udiv(WIDTH, HEIGHT);
          let transform_y = -1.0*(2.0*(y as f32 + 0.5 + FOURX_AA[i].y)/(HEIGHT as f32) - 1.0)*(fov/2.0).tan();
          let direction = Vector3::new(transform_x, transform_y, -1.0).normalize();
          color = color + (cast_ray(Vector3::new(0.0, 0.0, 0.0), direction, &*spheres_clone, &*lights_clone, 0)) * (1.0/(FOURX_AA.len() as f32));
        }
        //Send pixel back to main thread for assembly.
        tx.send((color, x+y*WIDTH)).unwrap();
      }
    }
    });
    //Push newly created thread.
    handles.push(handle);
  }
  drop(tx);
  let mut count: i32 = 0;
  
  //Main thread waits for new pixels and assembles them to the framebuffer.
  while let Ok(msg) = rx.recv() {
    count = count + 1;
    let (data, id) = msg;
    framebuffer[id] = data;
    print!("\r{:?}% of the image rendered.", (((count as f32)/((HEIGHT*WIDTH) as f32))*100.0) as i32);
  }
  
  //Kill the threads once finished.
  for handle in handles {
    handle.join().unwrap();
  }
  
  //Wrap everything up and send it to be output!
  let duration = start_time.elapsed();
  println!("\nRendering completed in {} seconds.", duration.as_secs_f64());
  let _ = framebuffer_to_ppm(&mut framebuffer);
}

fn main(){
  let temp = Model::new("res/monkey.obj");
  println!("{:?}", temp.faces);
  println!("{:?}", temp.verts);
  /*
  //Initialize some materials, lights, and objects.
  let shiny = Material::new(Vector3::new(0.4, 0.3, 0.4), Vector4::new(0.6, 0.3, 0.1, 0.0), 60.0, 1.0);
  let dull = Material::new(Vector3::new(0.1, 0.3, 0.1), Vector4::new(0.9, 0.1, 0.0, 0.0), 10.0, 1.0);
  let mirror = Material::new(Vector3::new(1.0, 1.0, 1.0), Vector4::new(0.0, 10.0, 0.8, 0.0), 1400.0, 1.0);
  let glass = Material::new(Vector3::new(0.6, 0.7, 0.8), Vector4::new(0.0,  0.5, 0.1, 0.8), 125.0, 1.5);

  let mut lights: Vec<Light> = Vec::new();
  lights.push(Light::new(Vector3::new(-20.0, 20.0, 20.0), 1.5));
  lights.push(Light::new(Vector3::new(30.0, 50.0, -25.0), 1.8));
  lights.push(Light::new(Vector3::new(30.0, 20.0, 30.0), 1.7));

  let mut spheres: Vec<Sphere> = Vec::new();
  spheres.push(Sphere::new(Vector3::new(-3.0, 0.0, -16.0), 3.0, shiny));
  spheres.push(Sphere::new(Vector3::new(-1.0, -1.5, -12.0), 1.0, glass));
  spheres.push(Sphere::new(Vector3::new(0.5, -0.5, -20.0), 2.0, dull));
  spheres.push(Sphere::new(Vector3::new(7.0, 5.0, -18.0), 4.0, mirror));
  
  //Begin the render!
  println!("Welcome to Rustracer, beginning your render...");
  render(&spheres, &lights);
  */
}
