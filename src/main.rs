extern crate argparse;
extern crate glob;
extern crate image;

use argparse::{ArgumentParser, Store};
use glob::glob;
use image::{GenericImageView, GenericImage, ImageBuffer, Rgba};
use std::path::Path;

fn main() {
    let mut input_dir = String::new();
    let mut output_dir = String::new();
    
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A script for converting images to grayscale.");
        ap.refer(&mut input_dir).required().add_option(&["--input-dir"], Store, "Directory of input images");
        ap.refer(&mut output_dir).required().add_option(&["--output-dir"], Store, "Directory to store output images");
        ap.parse_args_or_exit();
    }

    for entry in glob(&format!("{}/*.png", input_dir)).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let img = image::open(&path).unwrap().to_rgba8();

                let (width, height) = img.dimensions();

                let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

                for (x, y, pixel) in img.enumerate_pixels() {
                    let mut new_pixel = *pixel;

                    if new_pixel[3] == 0 {
                        new_pixel = Rgba([255, 255, 255, 255]);
                    }

                    buffer.put_pixel(x, y, new_pixel);
                }

                let gray_img = image::imageops::grayscale(&buffer);
                
                gray_img.save(format!("{}/{}", output_dir, path.file_name().unwrap().to_str().unwrap())).unwrap();
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
