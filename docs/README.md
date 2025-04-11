# Rustracer
A custom software raytraced 3D renderer that uses no external libraries or frameworks for its rendering model. I'm writing it to better understand graphics programming, low-level systems, and Rust.

## Features:
-Raytraced reflections, refraction, and shadows.\
-Multithreading support.\
-Diffuse lighting.\
-Specular illumination.\
-Anti-Aliasing\
-PPM output.\
-".obj" input.\
-Fast low-level performance.

## Compilation:
Install [Rust](https://www.rust-lang.org/tools/install) and then run "rustc main.rs -o rustracer" for Linux or "rustc main.rs -o rustracer.exe" on Windows.

## Project Structure:
src/main.rs <- This is what runs the raytracing calculations and rendering.\
src/definitions.rs <- This defines the data and geometry that is used for rendering (Vector3, Lights, Materials, etc).\
docs/ <- This is where the documentation is stored.\
compile.sh <- This is a Linux shell script you may run to compile the project.\
out.ppm <- Will be the output of the renderer, easily viewable in [GIMP](https://www.gimp.org/downloads/).

## Sample Output:
![A raytraced render from Rustracer.](out.png "Render")
