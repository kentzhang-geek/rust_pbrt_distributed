use gli_rs::{Format, GliTexture};
use super::math::*;
use image::{RgbaImage, Rgba32FImage, Rgb32FImage};
use ndarray::ShapeBuilder;
use crate::core::tools::PrintSelf;

/// film that stores rendering result
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
    /// create a new film, pixel type will always be 4 double precision floats
    pub fn new(width: u32, height: u32) -> Film {
        let ret = Film {
            fullResolution: (width, height),
            pixels: Box::new(ndarray::Array::default((width as usize, height as usize))),
        };
        return ret;
    }
    /// get one value from this film, return None if failed
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
    /// store one value to this film, return false if failed
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
    /// save this film to an exr image file
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
    /// save this film to a png image file
    pub fn save_png(&self, mut filename: String)->image::ImageResult<()> {
        if !filename.ends_with(".png") {
            filename = filename + ".png";
        }
        let mut img = RgbaImage::new(self.fullResolution.0, self.fullResolution.1);
        for ridx in 0..self.fullResolution.0 {
            for cidx in 0..self.fullResolution.1 {
                let mut pix_film = self.pixels.get((ridx as usize, cidx as usize)).unwrap().clone();
                pix_film.rgba = pix_film.rgba * 255.0f32;   // from 0 ~ 255 in png
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
    /// save this film to a dds image file
    ///
    /// ------------
    /// ## USE THIS to Debug Image of Linear Color Space
    pub fn save_dds(&self, mut filename:String)->gli_rs::Result<()>{
        if !filename.ends_with(".dds") {
            filename = filename + ".dds";
        }
        let mut img = gli_rs::Texture2D::new(
            Format::RGBA32_SFLOAT_PACK32,
            gli_rs::Extent2d{ width: self.fullResolution.0, height: self.fullResolution.1 },
            1
        );

        for ridx in 0..self.fullResolution.0 {
            for cidx in 0..self.fullResolution.1 {
                let mut pix_film = self.pixels.get((ridx as usize, cidx as usize)).unwrap();
                let pix = [pix_film.rgba.x, pix_film.rgba.y, pix_film.rgba.z, pix_film.rgba.w];
                img.store(gli_rs::Extent2d{ width: ridx, height: cidx }, 0, pix);
            }
        }

        return gli_rs::save_dds(&img, filename);
    }
}

