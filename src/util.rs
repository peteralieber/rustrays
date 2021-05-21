// util.rs - Utility functions for simple ray tracer

pub fn output_image() {
    // image properties
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    const DYN_RANGE: i32 = 256;
    const MAX_VAL: f32 = DYN_RANGE as f32 - 0.001;
    
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

            let ir = MAX_VAL * r;
            let ig = MAX_VAL * g;
            let ib = MAX_VAL * b;

            println!("{} {} {}", ir as u32, ig as u32, ib as u32);
        }
    }
}