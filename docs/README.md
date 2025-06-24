# Rustracer
A custom software raytraced 3D renderer that uses no external libraries or frameworks for its rendering model. It also has a custom scripting language for setting up your scenes. I'm writing it to better understand graphics programming, low-level systems, and Rust.

## Compilation:
Install [Rust](https://www.rust-lang.org/tools/install) and then run "rustc main.rs -o rustracer" for Linux or "rustc main.rs -o rustracer.exe" on Windows.

## Features:
- Raytraced reflections, refraction, and shadows.
- Multithreading support.
- Custom script interpreter.
- Diffuse lighting.
- Specular illumination.
- Anti-Aliasing
- PPM output.
- OBJ input.
- Fast low-level performance.

## Sample Output:
![A raytraced scene from Rustracer.](out.png "Render")\
![A raytraced monkey from Rustracer.](monkey.png "Render")\
![A raytraced teapot from Rustracer.](tea.png "Render")

## Project Structure:
src/main.rs <- This is what runs the raytracing calculations and rendering.\
src/intepreter.rs <- This interprets the input script and turns it into understandable instructions for the renderer.\
src/model.rs <- This parses and sets up our 3D models that we've fed in as OBJ files.\
src/definitions.rs <- This defines the data and geometry that is used for rendering (Vector3, Lights, Materials, etc).\
docs/ <- This is where the documentation is stored.\
res/ <- This is where the models are stored.\
scripts/ <- This is where example scripts for the raytracer to run are stored.\
compile.sh <- This is a Linux shell script you may run to compile the project.\
out.ppm <- Will be the output of the renderer, easily viewable in [GIMP](https://www.gimp.org/downloads/).\
![A sequence diagram of Rustracer.](sequence_diagram.png "Sequence Diagram")

## RT Script:
Check out the example scripts as they exhaust the full syntax of this language, make sure to define the materials before you define meshes or spheres.
