#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod image_processing;
pub mod nodes;
pub mod types;
pub mod utils;
pub use app::NodeGraphExample;
