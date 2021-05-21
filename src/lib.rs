// lib.rs
pub mod util;
pub mod vectors;
pub mod colors;
pub mod rays;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_image() {
        util::output_image();
    }
}
