//! This project is a Rust port of an original Ada project, which was created as a
//! companion to the blog post:
//! **"Introduction to Ada: a project-based exploration with rosettas."**
//!
//! The original Ada source code can be found in its respective repository.
//!
//! ## Enhancements
//!
//! This version is not just a direct translation; it includes several improvements:
//! - Cleaner SVG output file generation.
//! - Additional fun graphical effects, powered by CSS.
//!
//! ## Future Work
//!
//! To further improve the codebase, future versions could:
//! - Calculate the number of revolutions for the hypotrochoid curve based on its
//!   parameters, instead of using a hard-coded value.
//! - Integrate the `indoc` crate to manage multi-line raw strings more cleanly.
//! - Use the `svg` crate to build the SVG document programmatically instead of
//!   using raw strings, which would make the code more robust and maintainable.

mod rosetta;
mod renderer;

/// The main function and entry point of the program.
fn main() {
    // Renders a predefined set of animated rosettas into a SVG output.
    if let Err(e) = renderer::create_svg_rosettas() {
        eprintln!("ERROR: Failed to generate SVG file: {}", e);
    }
}
