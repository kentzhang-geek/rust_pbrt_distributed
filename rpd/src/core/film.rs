use super::math::*;
use image::{RgbaImage, Rgba32FImage};
use ndarray::ShapeBuilder;
use crate::core::tools::PrintSelf;

#[derive(Debug, Clone, Default)]
pub struct Film {
    pub fullResolution: (u32, u32),
    pub pixels: Box<ndarray::Array2<Pixel>>,
}

/// Film's pixel, data type of elements in film
#[derive(Debug, Clone, Default)]
pub struct Pixel {
    pub rgba: Vector4f,
}

impl Film {
    pub fn new(width: u32, height: u32) -> Film {
        let ret = Film {
            fullResolution: (width, height),
            pixels: Box::new(ndarray::Array::default((width as usize, height as usize))),
        };
        return ret;
    }
    pub fn get(&self, pixel_coord: Point2i) -> Option<&Pixel> {
        let (x, y) = (pixel_coord.x as u32, pixel_coord.y as u32);
        if x < 0 || x >= self.fullResolution.0 {
            return None;
        }
        if y < 0 || y >= self.fullResolution.0 {
            return None;
        }
        return self.pixels.get((x as usize, y as usize));
    }
    pub fn set(& mut self, pixel_coord: Point2i, pix: &Pixel) -> bool {
        let (x, y) = (pixel_coord.x as u32, pixel_coord.y as u32);
        if x < 0 || x >= self.fullResolution.0 {
            return false;
        }
        if y < 0 || y >= self.fullResolution.0 {
            return false;
        }
        *self.pixels.get_mut((x as usize, y as usize)).unwrap() = pix.clone();
        return true;
    }
    pub fn save_exr(&self, mut filename: String)->image::ImageResult<()> {
        if !filename.ends_with("exr") {
            filename = filename + ".exr";
        }
        let mut img = Rgba32FImage::new(self.fullResolution.0, self.fullResolution.1);
        for ridx in 0..self.fullResolution.0 {
            for cidx in 0..self.fullResolution.1 {
                let pix_film = self.pixels.get((ridx as usize, cidx as usize)).unwrap();
                let pix_sto: image::Rgba<f32> = image::Rgba {
                    0: [
                        pix_film.rgba.x, pix_film.rgba.y, pix_film.rgba.z, pix_film.rgba.w
                    ]
                };
                img.put_pixel(ridx, cidx, pix_sto);
            }
        }
        return img.save(filename);
    }
    pub fn save_png(&self, mut filename: String)->image::ImageResult<()> {
        if !filename.ends_with(".png") {
            filename = filename + ".png";
        }
        let mut img = RgbaImage::new(self.fullResolution.0, self.fullResolution.1);
        for ridx in 0..self.fullResolution.0 {
            for cidx in 0..self.fullResolution.1 {
                let mut pix_film = self.pixels.get((ridx as usize, cidx as usize)).unwrap().clone();
                pix_film.rgba = pix_film.rgba * 255.0f32;
                let pix_sto: image::Rgba<u8> = image::Rgba {
                    0: [
                        pix_film.rgba.x as u8, pix_film.rgba.y as u8, pix_film.rgba.z as u8, pix_film.rgba.w as u8
                    ]
                };
                img.put_pixel(ridx, cidx, pix_sto);
            }
        }
        return img.save(filename);
    }
}

