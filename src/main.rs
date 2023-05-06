use std::path::Path;
use std::sync;
use std::thread;

use tinyraytracer::camera::ASPECT_RATIO;
use tinyraytracer::camera::Camera;
use tinyraytracer::shape::hittable::HitRecord;
use tinyraytracer::{
    ray::Ray,
    shape::{hittable::Hittable, hittable_list::HittableList, sphere::Sphere},
    vec3::Vector3,
    Scalar,
};

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as Scalar / ASPECT_RATIO) as u32;

const SAVE_PATH: &str = "image.png";

const NUM_THREADS: usize = 4;

fn send_ray(ray: &Ray, world: &HittableList) -> Vector3 {
    let mut rec = HitRecord::new();
    if world.hit(&ray, 0.0, f32::INFINITY, &mut rec) {
        return (rec.normal + Vector3::new(1.0)) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    let sky_start = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let sky_end = Vector3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };

    sky_start * (1.0 - t) + sky_end * t
}

fn main() {
    let mut buffer = vec![[Vector3::new(0.0); IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize];

    /* Camera stuff */
    let camera = Camera::new();

    /* World */

    struct Worker {
        pixel_position: (usize, usize),
        tx: sync::mpsc::Sender<Ray>,
        rx: sync::mpsc::Receiver<Vector3>,
        status: bool,
    }

    impl Worker {
        fn new(tx: sync::mpsc::Sender<Ray>, rx: sync::mpsc::Receiver<Vector3>) -> Self {
            Worker {
                pixel_position: (0, 0),
                tx,
                rx,
                status: true,
            }
        }
    }

    /* Create threads */
    let mut workers = Vec::new();
    let mut threads = Vec::new();
    for _ in 0..NUM_THREADS {
        let (tx_ray, rx_ray) = sync::mpsc::channel::<Ray>();
        let (tx_color, rx_color) = sync::mpsc::channel::<Vector3>();
        let worker = Worker::new(tx_ray, rx_color);

        workers.push(worker);
        threads.push(thread::spawn(move || {
            let mut world = HittableList::new();
            world.add(Box::new(Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                0.5,
            )));
            world.add(Box::new(Sphere::new(
                Vector3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0,
                },
                100.0,
            )));

            while let Ok(ray) = rx_ray.recv() {
                let pixel_color = send_ray(&ray, &world);
                tx_color.send(pixel_color).unwrap();
            }
        }));
    }

    let mut x: usize = 0;
    let mut y: usize = 0;

    /* Send first ray to workers */
    println!("Sending rays");
    for worker in &mut workers {
        worker.pixel_position = (x, y);
        let u = x as Scalar / (IMAGE_WIDTH - 1) as Scalar;
        let v = ((IMAGE_HEIGHT - 1) - y as u32) as Scalar / (IMAGE_HEIGHT - 1) as Scalar;

        let ray = camera.get_ray(u, v);

        worker.tx.send(ray).unwrap();

        x += 1;
        if x >= IMAGE_WIDTH as usize {
            x = 0;
            y += 1;
        }
    }

    while workers.iter().any(|w| w.status == true) {
        /* Main loop */
        for worker in &mut workers {
            if let Ok(pixel_color) = worker.rx.try_recv() {
                buffer[worker.pixel_position.1][worker.pixel_position.0] = pixel_color;

                if y < IMAGE_HEIGHT as usize {
                    worker.pixel_position = (x, y);

                    let u = x as Scalar / (IMAGE_WIDTH - 1) as Scalar;
                    let v =
                        ((IMAGE_HEIGHT - 1) - y as u32) as Scalar / (IMAGE_HEIGHT - 1) as Scalar;

                    let ray = camera.get_ray(u, v);

                    worker.tx.send(ray).unwrap();

                    x += 1;
                    if x >= IMAGE_WIDTH as usize {
                        x = 0;
                        y += 1;
                    }
                } else {
                    worker.status = false;
                }
            }
        }
    }
    println!("Done");

    /*
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
    */

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
