# Rendering

The project uses WebGPU for high-performance rendering, providing both low-level pipeline access and a high-level 2D drawing API.

## Canvas API

The `Canvas` class provides a simplified API for 2D rendering, which is ideal for UI, debugging, and simple games. It supports:
- Drawing rectangles with colors.
- Rendering text using the `Glyphon` library.
- Coordinate transformations (translation, rotation, scaling) via `Mat3`.

## GPU Pipeline

For 3D rendering or complex effects, the project provides a `RenderPipelineBuilder` that simplifies the creation of `wgpu::RenderPipeline`.

## Coordinate System
The rendering system uses a normalized coordinate system where necessary, but the `Canvas` API typically operates in pixel coordinates relative to the window size.
