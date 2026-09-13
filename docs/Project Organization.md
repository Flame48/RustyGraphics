# Project Organization

This is a short description of how the project is currently organized.

## Main Applications

There are currently 2 main applications or builds,

1. graphics-cli: A more in-depth cli application that supports more advanced keyword applications.

2. graphics-view: A simple executable file that can be used to quickly preview supported files for 3D meshes.

## Project Structure

The overall project is divided into 2 different layers:

1. Application Layer: Handles logic regarding the user's inputs, and outputing results onto the screen. The abstraction allows for a single application to be run in different environments, eg. in a windowed gui vs as a cli in the terminal.

2. Scene Layer: Handles logic for tracking and describing a 3D scene, providing methods for manipulating the objects within the scene tree.

3. Renderer Layer: Handles the rendering of a 3D scene to a screen.

You can find most of the application layer logic [here](../src/application/), and the scene and rendering layers [here](../src/scene/).
