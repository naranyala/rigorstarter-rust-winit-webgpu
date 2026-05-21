# Project Roadmap

## ✅ Completed

### Core Engine
- [x] WebGPU abstraction layer (`GpuContext`)
- [x] High-level `Canvas` API for immediate-mode drawing
- [x] Batching system for efficient primitive rendering
- [x] Procedural bit-style font renderer

### Snake Game
- [x] Game logic (movement, collision, food)
- [x] Grid-based rendering with clear cell borders
- [x] Adjustable tick rate for gameplay speed

### Launcher UI
- [x] Fuzzy search implementation
- [x] Pixel-perfect UI layout
- [x] Integration of bit-style font for all UI elements
- [x] State machine for Launcher $\leftrightarrow$ Game transitions

## 🚀 Upcoming Features

### 🎮 More Demos
- [ ] **Triangle/Geometry Demo**: Showcases raw pipeline and vertex buffer manipulation.
- [ ] **Particle System**: Demonstrs compute shaders and large-scale vertex updates.
- [ ] **Texture/Sprite Demo**: Demonstrs texture loading and UV mapping.

### 🛠️ Engine Enhancements
- [ ] **Advanced UI System**: Support for buttons, checkboxes, and sliders.
- [ ] **Advanced Font Rendering**: Integration of `glyphon` for high-quality anti-aliased text (toggleable).
- [ ] **Camera System**: Support for 3D transformations and camera matrices.
- [ ] **Sound Engine**: Integration of audio for game feedback.

### 🔧 Developer Experience
- [ ] **Asset Pipeline**: Automatic loading of textures and fonts.
- [ ] **Hot Reloading**: Live-reloading of shaders and assets.
