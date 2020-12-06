/// A simple, almost no dependency CPU-Voxelizer.
/// It supports surface and solid voxelization.
pub(crate) mod sat;
pub(crate) mod vector;
pub mod voxelize;

pub use voxelize::*;
