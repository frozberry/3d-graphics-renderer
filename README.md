# 3D Graphics Renderer Rust

A 3D graphics renderer built with Rust and SDL2.
https://user-images.githubusercontent.com/93860959/186969981-6ae56386-2ec5-4f17-8cd0-a2822d722d35.mp4

### Features
  - Flat light shading
  - Back face culling
  - Multiple render modes
  - Loading from OBJ files
  - Colored polygon faces
  - Matrix transformations

## Install

Install system dependencies
```
<!-- Linux -->
sudo apt install libsdl2-dev
sudo apt install lib2sdl-image-dev
sudo apt instal lib2sdl-gfx-dev

<!-- Mac -->
sudo apt install libsdl2-dev
sudo apt install lib2sdl-image-dev
sudo apt instal lib2sdl-gfx-dev
```
Clone and run
```
git clone git@github.com:frozberry/3d-graphics-renderer.git
cd 3d-graphics-renderer 
cargo run --release
```

`1` Fill polygon mode

`2` Fill polygon with wireframe

`3` Wireframe

`4` Wireframe with textures 

`c` Toggle back face culling

`t` Toggle model

`p` Pause
