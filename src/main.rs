use std::path::Path;

type Scalar = f32;

const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: Scalar = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as Scalar / ASPECT_RATIO) as u32;
const SAVE_PATH: &str = "image.png";

#[repr(C)]
#[derive(Copy, Clone)]
struct Color {
    r: Scalar,
    g: Scalar,
    b: Scalar,
}

fn main() {
    let buffer = [[Color {r: 0.52, g: 0.52, b: 0.52}; IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];

    println!("Creating image");
    let mut image_buffer = [0u8; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];

    for y in 0usize..IMAGE_HEIGHT as usize {
        for x in 0usize..IMAGE_WIDTH as usize {
            image_buffer[(y * IMAGE_WIDTH as usize + x) * 3] = (buffer[y][x].r * 255.999).clamp(0.0, 255.0) as u8;
            image_buffer[(y * IMAGE_WIDTH as usize + x) * 3 + 1] = (buffer[y][x].g * 255.999).clamp(0.0, 255.0) as u8;
            image_buffer[(y * IMAGE_WIDTH as usize + x) * 3 + 2] = (buffer[y][x].b * 255.999).clamp(0.0, 255.0) as u8;
        }
    }

    println!("Saving image as {SAVE_PATH}");
    if let Err(error) = image::save_buffer(
        &Path::new(SAVE_PATH),
        &image_buffer,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        image::ColorType::Rgb8,
    ) {
        println!("An error occured while saving image error message: {}", error.to_string());
    }
}
