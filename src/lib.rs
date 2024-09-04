//! _nice_and_easy_ is a Rust library designed to make smooth transitions a breeze.
//!
//! Whether you’re working with f32 or f64 types, this library offers a wide range of easing functions that help you create fluid animations and transitions in your applications.
//!
//! From linear to more complex easing functions, _nice_and_easy_ has you covered, allowing you to focus on crafting delightful user experiences without the hassle.
//!
//! Features:
//! - Comprehensive collection of easing functions.
//! - Seamless support for both f32 and f64 types
//! - Lightweight and easy to integrate into any Rust project.
//! - Perfect for game development, UI animations, and more.
//!
//! With _nice_and_easy_, achieving smooth, natural motion has never been _easier_!
//!
//! For visualizing these functions, you can use the website <https://easings.net>
//!
//! - t is the progress of the tween, ranging from 0 to d.
//! - b is the starting value of the property.
//! - c is the change between the starting and ending value of the property.
//! - d is the total duration
//!
//! ```rust
//! use nice_and_easy::*;
//!
//! fn main() {
//!     // Can be used with f32...
//!     let progress: f32 = 0.5;
//!     let starting_value: f32 = 0.0;
//!     let target: f32 = 2.0;
//!     let duration: f32 = 1.0;
//!     let value: f32 = sine_in_out(progress, starting_value, target, duration);
//!     assert_eq!(value, 1.0);
//!
//!     // ...or f64
//!     let progress: f64 = 0.5;
//!     let starting_value: f64 = 0.0;
//!     let target: f64 = 2.0;
//!     let duration: f64 = 1.0;
//!     let value: f64 = quad_in_out(progress, starting_value, target, duration);
//!     assert_eq!(value, 1.0);
//! }
//!
//! ```

mod easing;

pub use easing::*;
