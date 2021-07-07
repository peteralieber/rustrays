// Render
use super::cameras::*;
use super::colors::*;
use super::primitives::*;
use super::util::*;
use super::rays::*;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;
const DYN_RANGE: u32 = 256;

const INFINITY: f32 = std::f32::INFINITY;
const PI: f32 = 3.1415926535897932385;

pub struct SceneConfig {
    aspect_ratio: f32,
    image_width: u32,
    image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub dyn_range: u32,
}

impl SceneConfig {
    pub fn new() -> Self {
        Self {
            aspect_ratio: ASPECT_RATIO,
            image_width: IMAGE_WIDTH,
            image_height: IMAGE_HEIGHT,
            samples_per_pixel: SAMPLES_PER_PIXEL,
            max_depth: MAX_DEPTH,
            dyn_range: DYN_RANGE,
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.image_width = width;
        self.image_height = (width as f32 / self.aspect_ratio) as u32;
    }
}

pub fn render_image_png(scene: &SceneConfig, world: &HittableList, cam: &Camera, filename: &str) {
    // Render Image
    
    let path = Path::new(filename);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, scene.image_width, scene.image_height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let mut data: Vec<u8> = Vec::new();

    for j in (0..scene.image_height).rev() {
        //std::io::stderr().write_fmt("\nScanlines remaining: {} ", j);
        //eprintln!("\nScanlines remaining: {} ", j);
        for i in 0..scene.image_width {
            /*let u = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r = Ray {origin: origin, direction: (lower_left_corner + u*horizontal + v*vertical - origin)};

            let pixel_color = ray_color_normals(&r, &world);*/
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _s in 0..scene.samples_per_pixel {
                let u = (i as f32 + rand())/(scene.image_width as f32 - 1.0);
                let v = (j as f32 + rand())/(scene.image_height as f32 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color_bounce(&r, world, scene.max_depth);
            }
            pixel_color /= scene.samples_per_pixel as f32;
            pixel_color.gamma_correct();
            let png_color = pixel_color.get_png_color();
            for val in png_color {
                data.push(val);
            }
        }
    }

    writer.write_image_data(&data).unwrap();
}

pub fn render_image_ppmstdout(scene: &SceneConfig, world: &HittableList, cam: &Camera, filename: &str) {
    // Render Image
    
    println!("P3");
    println!("{} {}", scene.image_width, scene.image_height);
    println!("{}", (scene.dyn_range-1) as u32);

    for j in (0..scene.image_height).rev() {
        //std::io::stderr().write_fmt("\nScanlines remaining: {} ", j);
        eprintln!("\nScanlines remaining: {} ", j);
        for i in 0..scene.image_width {
            /*let u = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r = Ray {origin: origin, direction: (lower_left_corner + u*horizontal + v*vertical - origin)};

            let pixel_color = ray_color_normals(&r, &world);*/
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _s in 0..scene.samples_per_pixel {
                let u = (i as f32 + rand())/(scene.image_width as f32 - 1.0);
                let v = (j as f32 + rand())/(scene.image_height as f32 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color_bounce(&r, world, scene.max_depth);
            }
            pixel_color /= scene.samples_per_pixel as f32;
            pixel_color.gamma_correct();
            println!("{}", pixel_color);
        }
    }
}

pub fn ray_color_bounce(r: &Ray, world: &impl Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0,0.0,0.0);
    }

    match world.hit(r, 0.001, INFINITY) {
        None => ray_color_bg(r),
        Some(hit_record) => {
            let target = rand_lamb_vector(hit_record);
            0.5 * ray_color_bounce(&Ray{origin:hit_record.p, direction:target-hit_record.p}, world, depth-1)
        },
    }
}