pub mod core;
pub mod input;
pub mod physics;
pub mod math;
pub mod state;
pub mod linear_algebra;

// Re-export for ergonomics
pub use core::Clock;
pub use input::InputManager;
pub use state::*;
pub use physics::*;
pub use math::*;
pub use linear_algebra::*;
