// --------------------    Core    --------------------
mod core;
pub use crate::core::*;
// ---------------------  Logging ----------------------
pub mod log;
// -------------------- Entry Point --------------------
#[macro_use]
mod entry_point;
// ------------------  Event System --------------------
mod event;
// ------------------ Window ------------------
mod window;

// ------------------ Platform Specific Code ------------------
mod platform;