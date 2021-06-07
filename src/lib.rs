// lib.rs
pub mod util;
pub mod vectors;
pub mod colors;
pub mod rays;
pub mod primitives;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_image() {
        util::output_color_gradient();
    }

    #[test]
    fn output_blue_white_gradient() {
        util::output_blue_white_gradient();
    }
}
