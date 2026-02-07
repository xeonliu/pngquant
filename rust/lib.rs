/*
** © 2024 by Kornel Lesiński.
**
** See COPYRIGHT file for license.
*/

// Core compression logic
pub mod core;

#[cfg(feature = "python")]
pub mod python_bindings;

#[cfg(feature = "python")]
pub use python_bindings::*;
