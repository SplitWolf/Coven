mod core;
pub use crate::core::Application;
// -------------------- Entry Point --------------------
#[macro_use]
mod entry_point;
// -----------------------------------------------------
#[macro_use]
pub mod log;
pub use crate::log::*;