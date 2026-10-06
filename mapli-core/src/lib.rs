mod encode;
mod error;
mod pool;
mod types;
mod worker;

pub use encode::encode_png;
pub use error::{MapliError, Result};
pub use pool::{PoolConfig, RenderPool};
pub use types::*;

// Re-export `RgbaImage` from the `image` crate so that users of this crate don't have to depend
// on `image` themselves.
pub use image::RgbaImage;

