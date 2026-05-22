# Rigorstarter Rust Winit WebGPU

A high-performance starter kit for building interactive applications, games, and mathematical visualizations using Rust, Winit, and WebGPU.

## 🚀 Overview

This project provides a robust foundation for GPU-accelerated applications. It combines a clean state-management system with a custom linear algebra library and a flexible 2D/3D rendering pipeline.

## ✨ Key Features

- **State-Driven Architecture**: A stack-based `StateManager` for seamless transitions between menus and games.
- **Custom Linear Algebra**: Lightweight `Vec2`, `Vec3`, `Mat3`, and `Mat4` implementations optimized for graphics.
- **Hybrid Rendering**:
  - **Canvas API**: High-level 2D drawing for UI and text.
  - **WebGPU Pipeline**: Low-level access for custom shaders and 3D rendering.
- **Integrated Launcher**: A searchable, scrollable menu to manage and launch multiple sub-projects/examples.

## 🛠️ Tech Stack

- **Language**: Rust (Edition 2024)
- **Windowing**: `winit`
- **Graphics**: `wgpu` (WebGPU)
- **Text Rendering**: `glyphon` & `cosmic-text`

## 📖 Documentation

Detailed guides are available in the `/docs` directory:
- [Architecture](docs/architecture.md) - The state pattern and manager.
- [Rendering](docs/rendering.md) - Canvas API and GPU pipelines.
- [Linear Algebra](docs/linear_algebra.md) - Math library details.
- [Launcher](docs/launcher.md) - How to extend the application.

## 🚦 Getting Started

### Prerequisites
- Rust (latest stable)
- WebGPU compatible drivers (Vulkan, Metal, or DX12)

### Running the App
```bash
cargo run
```
