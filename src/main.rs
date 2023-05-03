use std::path::Path;

use tinyraytracer::{ray::Ray, vec3::Vector3, Scalar};

const ASPECT_RATIO: Scalar = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as Scalar / ASPECT_RATIO) as u32;

const SAVE_PATH: &str = "image.png";

fn hit_sphere(center: &Vector3, radius: Scalar, ray: &Ray) -> Scalar {
    let oc = ray.origin - (*center);
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn send_ray(ray: &Ray) -> Vector3 {
    let sphere_center = Vector3 { x: 0.0, y: 0.0, z: -1.0};
    let t = hit_sphere(&sphere_center, 0.5, &ray);
    if  t > 0.0 {
        let normal = (ray.at(t) - sphere_center).unit_vector() + Vector3::new(1.0);
        return normal * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    let sky_start = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    let sky_end = Vector3 { x: 0.5, y: 0.7, z: 1.0 };

    sky_start * (1.0 - t) + sky_end * t
}

fn main() {
    let mut buffer = vec![[Vector3::new(0.0); IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];

    let viewport_height: Scalar = 2.0;
    let viewport_width: Scalar = viewport_height * ASPECT_RATIO;
    let focal_length: Scalar = 1.0;
    let origin: Vector3 = Vector3::new(0.0);
    let horizontal: Vector3 = Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical: Vector3 = Vector3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin - horizontal / 2.0
        - vertical / 2.0
        - Vector3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    for y in 0..buffer.len() {
        for x in 0..buffer[y].len() {
            let u = x as Scalar / (IMAGE_WIDTH - 1) as Scalar;
            let v = ((IMAGE_HEIGHT - 1) - y as u32) as Scalar / (IMAGE_HEIGHT - 1) as Scalar;

            let ray = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            );

            buffer[y][x] = send_ray(&ray);
        }
    }

    println!("Creating image");
    let mut image_buffer = [0u8; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];

    for y in 0usize..IMAGE_HEIGHT as usize {
        for x in 0usize..IMAGE_WIDTH as usize {
            image_buffer[(y * IMAGE_WIDTH as usize + x) * 3] =
                (buffer[y][x].x * 255.999).clamp(0.0, 255.0) as u8;
            image_buffer[(y * IMAGE_WIDTH as usize + x) * 3 + 1] =
                (buffer[y][x].y * 255.999).clamp(0.0, 255.0) as u8;
            image_buffer[(y * IMAGE_WIDTH as usize + x) * 3 + 2] =
                (buffer[y][x].z * 255.999).clamp(0.0, 255.0) as u8;
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
        println!(
            "An error occured while saving image error message: {}",
            error.to_string()
        );
    }
}
