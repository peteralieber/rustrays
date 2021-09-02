
use rustrays::util::*;
use rustrays::*;
use log::*;

fn main() {
    //set_max_level(LevelFilter::Info);
    //println!("RustRays!");
    
    //output_color_gradient();
    //output_blue_white_gradient();
    //output_sphere_on_sphere();
    output_metal_spheres();

    let v1 = vectors::Vector3 {x:1.0,y:1.0,z:1.0};
    let v2 = vectors::Vector3 {x:1.0,y:2.0,z:3.0};
    let v3 = vectors::Vector3 {x:2.0,y:2.0,z:2.0};

    info!("Vector (1,1,1) + (1,2,3) = {}", v1+v2);
    info!("Vector (2,2,2) * (1,2,3) = {}", v3*v2);
    info!("Vector (2,2,2) . (1,2,3) = {}", v3.dot(v2));
    info!("Vector (2,2,2) x (1,2,3) = {}", v3.cross(v2));

    eprintln!("Done!");
}
