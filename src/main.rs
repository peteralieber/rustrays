
mod util;
mod vectors;

fn main() {
    //println!("RustRays!");
    
    //util::output_image();

    let v1 = vectors::Point3 {x:1.0,y:1.0,z:1.0};
    let v2 = vectors::Point3 {x:1.0,y:2.0,z:3.0};
    let v3 = vectors::Point3 {x:2.0,y:2.0,z:2.0};

    eprintln!("Vector (1,1,1) + (1,2,3) = {}", v1+v2);
    eprintln!("Vector (2,2,2) * (1,2,3) = {}", v3*v2);
    eprintln!("Vector (2,2,2) . (1,2,3) = {}", v3.dot(v2));
    eprintln!("Vector (2,2,2) x (1,2,3) = {}", v3.cross(v2));
}
