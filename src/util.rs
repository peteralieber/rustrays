// util.rs - Utility functions for simple ray tracer
use super::colors::*;
use super::vectors::*;
use super::rays::*;
use super::primitives::*;
use super::cameras::*;
use super::materials::*;
use rand::prelude::*;
use super::render::*;
use Vector3 as Point3;

const INFINITY: f32 = std::f32::INFINITY;
const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

// Produce a random float [0,1)
pub fn rand() -> f32 {
    thread_rng().gen_range(0.0..1.0)
}

// Produce a random float [min,max)
pub fn rand_range(min:f32, max:f32) -> f32 {
    thread_rng().gen_range(min..max)
}

pub fn rand_in_unit_sphere() -> Vector3 {
    let mut p = Vector3::rand_range(-1.0, 1.0);
    while p.length_squared() >= 1.0 {
        p = Vector3::rand_range(-1.0, 1.0);
    };
    p
}

pub fn rand_lamb_vector(hit_record: &HitRecord) -> Vector3 {
    hit_record.normal + rand_in_unit_sphere().unit_vector()
}

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

pub fn ray_color_bg(r: &Ray) -> Color {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5*(unit_direction.y + 1.0);
    (1.0-t)*Color{r:1.0, g:1.0, b:1.0} + t*Color{r:0.5, g:0.7, b:1.0}
}

pub fn output_blue_white_gradient() {
    // image properties
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const MAX_DEPTH: u32 = 50;
    
    // Camera

    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Vector3 {x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vector3 {x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vector3 {x: 0.0, y: viewport_height, z: 0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vector3{x:0.0,y:0.0,z:focal_length};

    // Materials
    let material_white: Material = Default::default();

    // Sphere
    let sphere1 = Sphere{center: Vector3::new(0.0, 0.0, -1.0), material: &material_white, radius: 0.5};

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

            let pixel_color = ray_color_bounce(&r, &sphere1, MAX_DEPTH);

            println!("{}", pixel_color);
        }
    }
}

pub fn output_sphere_on_sphere() {

    // Materials
    let material_white: Material = Default::default();

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere{center: Vector3::new(0.0,0.0,-1.0), material: &material_white, radius: 0.5}));
    world.add(Box::new(Sphere{center: Vector3::new(0.0,-100.5,-1.0), material: &material_white, radius: 100.0}));
    
    // Camera

    let cam = Camera::new(Point3::new(0.0,0.0,0.0), Point3::new(0.0,0.0,-1.0), Point3::new(0.0,1.0,0.0), 60.0, 16.0/9.0, 1.0, 1.0);

    // Scene Config
    let mut scene = SceneConfig::new();
    scene.set_width(600);
    scene.samples_per_pixel = 200;

    // Render Image
    
    //render_image_ppmstdout(&scene, &world, &cam);
    render_image_png(&scene, &world, &cam, "image.png");
}



pub fn output_metal_spheres() {

    // Materials
    let material_ground = Material::Diffuse {albedo: Color::new(0.8, 0.8, 0.0)};
    let material_center = Material::Diffuse {albedo: Color::new(0.1, 0.2, 0.5)};
    let material_left = Material::Dialectric {albedo: Color::new(1.0, 1.0, 1.0), index_of_refraction: 1.5};
    let material_right = Material::Metal {albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.005};

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere{center: Vector3::new(0.0,-100.5,-1.0), material: &material_ground, radius: 100.0}));
    world.add(Box::new(Sphere{center: Vector3::new(0.0, 0.0, -1.0), material: &material_center, radius: 0.5}));
    world.add(Box::new(Sphere{center: Vector3::new(-1.0, 0.0, -1.0), material: &material_left, radius: 0.5}));
    world.add(Box::new(Sphere{center: Vector3::new(-1.0, 0.0, -1.0), material: &material_left, radius: -0.49}));
    world.add(Box::new(Sphere{center: Vector3::new(1.0, 0.0, -1.0), material: &material_right, radius: 0.5}));
    
    // Camera

    let lookfrom = Point3::new(-2.0,2.0,1.0);
    let lookat = Point3::new(0.0,0.0,-1.0);
    let vup = Point3::new(0.0,1.0,0.0);

    let focus_dist = (lookfrom - lookat).length();
    let aperture = 0.4;

    let cam = Camera::new(lookfrom, lookat, vup, 40.0, 16.0/9.0, aperture, focus_dist);

    // Scene Config
    let mut scene = SceneConfig::new();
    scene.set_width(800);
    scene.samples_per_pixel = 150;
    scene.max_depth = 20;

    // Render Image
    
    //render_image_ppmstdout(&scene, &world, &cam);
    render_image_png(&scene, &world, &cam, "image.png");
}

/// Color surface normals
pub fn ray_color_normals(r: &Ray, world: &impl Hittable) -> Color {
    match world.hit(r, 0.0, INFINITY) {
        None => ray_color_bg(r),
        Some(hit_record) => 0.5 * (Color::from_vector(hit_record.normal) + Color::new(1.0,1.0,1.0,)),
    }
}

/// Simple diffuse tracer
fn ray_color_bounce(r: &Ray, world: &impl Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0,0.0,0.0);
    }

    match world.hit(r, 0.001, INFINITY) {
        None => ray_color_bg(r),
        Some(hit_record) => {
            let target = rand_lamb_vector(&hit_record);
            0.5 * ray_color_bounce(&Ray{origin:hit_record.p, direction:target}, world, depth-1)
        },
    }
}