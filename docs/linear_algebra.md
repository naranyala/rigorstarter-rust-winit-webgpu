# Linear Algebra

The project includes a custom linear algebra library in `src/stdlib/linear_algebra` to avoid heavy dependencies and provide exactly what's needed for 2D/3D graphics.

## Vectors

- `Vec2`: Standard 2D vector with operations for length, normalization, dot product, and linear interpolation (LERP).
- `Vec3`: Standard 3D vector including cross product and normalization.

## Matrices

- `Mat3`: 3x3 matrix used primarily for 2D transformations.
- `Mat4`: 4x4 matrix used for 3D MVP (Model-View-Projection) transformations.

## Key Operations
All types implement basic operator overloading (`Add`, `Sub`, `Mul`, `Div`) and provide indexing for easy access to components.
