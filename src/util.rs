// util.rs - Utility functions for simple ray tracer
use super::colors::*;
use super::vectors::*;
use super::rays::*;

pub fn output_color_gradient() {
    // image properties
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    
    // Render Image
    
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", (DYN_RANGE-1) as u32);

    for j in (0..IMAGE_HEIGHT).rev() {
        //std::io::stderr().write_fmt("\nScanlines remaining: {} ", j);
        eprintln!("\nScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let g = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let b = 0.25;

            let pixel_color = Color {
                r: r,
                g: g,
                b: b
            };

            println!("{}", pixel_color);
        }
    }
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5*(unit_direction.y + 1.0);
    (1.0-t)*Color{r:1.0, g:1.0, b:1.0} + t*Color{r:0.5, g:0.7, b:1.0}
}

pub fn output_blue_white_gradient() {
    // image properties
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    
    // Camera

    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Vector3 {x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vector3 {x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vector3 {x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3{x:0.0,y:0.0,z:focal_length};

    // Render Image
    
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", (DYN_RANGE-1) as u32);

    for j in (0..IMAGE_HEIGHT).rev() {
        //std::io::stderr().write_fmt("\nScanlines remaining: {} ", j);
        eprintln!("\nScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r = Ray {origin: origin, direction: (lower_left_corner + u*horizontal + v*vertical - origin)};

            let pixel_color = ray_color_sphere(&r);

            println!("{}", pixel_color);
        }
    }
}

pub fn hit_sphere(center: &Vector3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = r.direction.dot(r.direction);
    let b = 2.0 * oc.dot(r.direction);
    let c = oc.dot(oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

pub fn ray_color_sphere(r: &Ray) -> Color {
    if hit_sphere(&Vector3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0,0.0,0.0);
    }
    ray_color(r)
}