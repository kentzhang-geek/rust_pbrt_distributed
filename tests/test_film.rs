#[cfg(test)]
mod tests_film {
    use rpd::interface::shape;
    use rpd::core::film;
    use rpd::core::math::*;
    use nalgebra_glm::*;
    use rpd::core::film::Pixel;
    use rpd::core::tools::PrintSelf;

    #[test]
    fn test01() {
        let mut film = film::Film::new(1024, 768);
        let pix_black = Pixel { rgba: Vector4f::new(0.0, 1.0, 1.0, 1.0) };
        for i in 0..1025 {
            film.set(Vector2i::new(i, 2i32), &pix_black);
            film.set(Vector2i::new(i, 3i32), &pix_black);
            film.set(Vector2i::new(i, 4i32), &pix_black);
            film.set(Vector2i::new(i, 5i32), &pix_black);
        }
        film.show_self();
        film.save_dds(String::from("target/test")).show_self();
    }
}
