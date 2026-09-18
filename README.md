# Terminal Graphics Renderer

> NOTE: This project is still a Work In Progress! Feel free to give feedback or otherwise reach out, contact information is listed [below](#contact) if you're interested.

This is a project I have wanted to work on for quite a while. It's a basic software renderer that renders 3D meshes and objects in the terminal with ASCII characters. This project is primarily for educational purposes as I wanted to both learn rust and familiarize myself more deeply with the mathematics and potential optimizations behind 3D rendering pipelines.

It can also be used in the commannd line to load and preview obj files, as shown below.

![Stanford Bunny Preview](<docs/images/Bunny Filled.png>)

![Cow Model Preview](<docs/images/Cow Filled.png>)

![Happy Buddha Statue](<docs/images/Happy Filled.png>)

The above models were sourced from [here](https://github.com/alecjacobson/common-3d-test-models/) and can be found in the [example folder](./examples/) for reference.

There's now also support for rendering in a window as opposed to the terminal for higher resolutions, mainly used for testing and performance analysis.

![Spinning Teapot Wireframe in Window example](<docs/images/Teapot Wireframe Demo Window.png>)

Currently, I am working on adding support for texture sampling with more complex scenes. Here is a sneak peak so far,

![Teapot with Checkered Texture applied and multiple colored lights](<docs/images/Teapot Lighting and Texture Development Demo.png>)

You can find instructions on the current setup and controls [here](#setup).

## Features

### Currently Supported

- Resizeable terminal screen display
- Scene tree system with hierarchical transformations
- Basic software rendering
- Camera movement controls
- Wireframe view
- Backface culling
- Importing .OBJ files
- Rendering in a window with higher resolution
- Computed flat normals
- Very Basic Lighting

### Currently Working On

- Texture Support
- Better lighting models
- Better materials
- Scene imports and exports

## Setup

Clone the repository and run with Cargo:

```bash
git clone https://github.com/Flame48/RustyGraphics.git
cd RustyGraphics
cargo build --release
```

For instruction on how to use run,

```bash
cargo run --release --bin graphics-cli -- -h
```

You can also run the exe file created in `./target/release/graphics-cli` with the specified flags.

There is also a separate build called `graphics-view` that can be used by windows to handle the opening of `.obj` and `.stl` files. Simply right click on a `.obj` / `.stl` file, select "Open With" and select the `target/release/graphics-view.exe`.

### Camera Controls

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
