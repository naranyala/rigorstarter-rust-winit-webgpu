# Rigorstarter Rust Winit WebGPU

A lightweight, high-performance WebGPU boilerplate and demonstration project written in Rust. It features a high-level immediate-mode `Canvas` API and a complete demo suite including a fuzzy-searchable launcher and a classic Snake game.

## ✨ Features

### 🏗️ Advanced WebGPU Abstraction
- **Low-Level API**: Direct access to `wgpu` primitives through a clean `GpuContext`.
- **High-Level `Canvas` API**: An immediate-mode drawing system inspired by `Raylib`.
  - `draw_rectangle(x, y, w, h, color)`
  - `draw_circle(cx, cy, r, color)`
  - `draw_text(text, x, y, size, color)` (Procedural Bit-style Font)
- **Batch Rendering**: Automatic vertex batching for high performance.

### 🕹️ Snake Game Demo
- **Procedural Grid**: Clear, pixel-perfect grid rendering.
- **Smooth Gameplay**: Adjustable tick rates for a classic, playable feel.
- **State Management**: Robust game loop with scoring and game-over states.

### 🔍 Fuzzy Search Launcher
- **Instant Search**: High-speed fuzzy matching for application items.
- **Bit-style UI**: A pixel-perfect, retro-inspired user interface.
- **Modern Text Rendering**: High-quality bit-style font rendering using procedural bit-mapping.

## 🚀 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/)
- [WGPU-compatible GPU Drivers](https://gpuinfo.org/) (Vulkan, Metal, or DX12)

### Installation & Running
1. Clone the repository:
   ```bash
   git clone <repo-url>
   cd rigorstarter-rust-winit-webgpu
   ```
2. Run the application:
   ```bash
   cargo run --release
   ```

## 🎮 Controls

### Launcher
- **Arrow Keys / WASD**: Navigate menu items
- **Enter**: Select item
- **Type**: Search through items
- **Escape**: Clear search query

### Snake Game
- **Arrow Keys / WASD**: Control snake direction
- **Space**: Restart game after Game Over
- **Escape**: Return to Launcher

## 📂 Project Structure

- `src/gpu/`: WebGPU abstraction layer and the `Canvas` drawing API.
- `src/game/`: Snake game logic and its specific renderer.
- `src/ui/`: Launcher state, fuzzy search logic, and UI rendering.
- `src/main.rs`: Application entry point and main state machine.
