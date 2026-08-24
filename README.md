# Terminal Graphics Renderer

> NOTE: This project is still a Work In Progress! Feel free to give feedback or otherwise reach out, contact information is listed [below](#contact) if you're interested.

This is a project I have wanted to work on for quite a while. It's a basic software renderer that renders 3D meshes and objects in the terminal with ASCII characters. This project is primarily for educational purposes as I wanted to both learn rust and familiarize myself more deeply with the mathematics and potential optimizations behind 3D rendering pipelines.

![Spinning Wireframe Cube example](<docs/images/Cube Wireframe Demo.png>)

![Spinning Solid Cube example (Note: There is currently no shading)](<docs/images/Cube Filled Demo.png>)

> Note: There is currently no shading supported. This will be added as an upcoming feature, [see below](#soon-to-be-added)

You can find instructions on setup and controls [here](#setup).

## Features

### Currently Supported

- Resizeable terminal screen display
- Scene tree system with hierarchical transformations
- Basic software rendering
- Camera movement controls
- Wireframe view
- Backface culling
- Importing .OBJ files

### Soon to be Added

- Computed normals
- Texture coordinates
- Basic lighting models

## Setup

Clone the repository and run with Cargo:

```bash
git clone https://github.com/Flame48/RustyGraphics.git
cd RustyGraphics
cargo run --release
```

### Controls

| Key         | Action                                                |
| ----------- | ----------------------------------------------------- |
| Esc         | Closes the application                                |
| W           | Moves the camera forward along it's local Z axis      |
| A           | Moves the camera to the left along it's local X axis  |
| S           | Moves the camera backwards along it's local Z axis    |
| D           | Moves the camera to the right along it's local X axis |
| Q           | Moves the camera up along the global Y axis           |
| E           | Moves the camera down along the global Y axis         |
| Z           | Switches the rendering to display wireframes          |
| Left Arrow  | Rotates the camera left around the global Y axis      |
| Right Arrow | Rotates the camera right around the global Y axis     |
| Up Arrow    | Rotates the camera up around the local X axis         |
| Down Arrow  | Rotates the camera down around the local X axis       |

# Contact

If you have any questions about the project or otherwise want to reach out, feel free to! My email is arajan8@wisc.edu
