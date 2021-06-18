# 1 RustRays Overview

![Current Render](/readme-files/image.png)

This is my attempt at following the excellent book by Peter Shirley, [Raytracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html), implemented in Rust.

I am a new Rustacean. In fact, this is my first Rust program. After starting the book without any Rust knowledge, I realized I wanted to implement this in the "Rust" way as much as possible.  Therefore, I took a pause from implementation, read [The Rust Book](https://doc.rust-lang.org/stable/book/), then started again.  

Where I've found a particular change necessary for Rust, or noticed some improvements by implementing in Rust, I note this in comments.

# 2 Output an Image

First up, is generating an image. In his C++ implementation, Peter S. uses static casting to output integer r, g, and b values with a dynamic range 0 - 255.999. In my implementation of a Color structure, I implement the Display trait to output PPM formated color pixel values. I have a DYN_RANGE and MAX_VAL constants to perform the normalization to a 0 - 255.999 range from 0.0 - 1.0.

```
pub const DYN_RANGE: i32 = 256;
const MAX_VAL: f32 = DYN_RANGE as f32 - 0.001;

#[derive(Copy, Clone, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", (self.r.clamp(0.0, 0.999)*MAX_VAL) as u32, (self.g.clamp(0.0, 0.999)*MAX_VAL) as u32, (self.b.clamp(0.0, 0.999)*MAX_VAL) as u32)
    }
}
```

# 3 The Vector3 Class

My first real "Idiomatic Rust" quandary was whether to implement the Vector3 and Color as separate structures or aliased, like the original author. This became apparent as I didn't like Rust's aliasing capabilities (made it seem clunky), and I wanted to get Color operations that made sense for colors, but as methods, rather than utility functions.  I had to implement many operations twice, which can be hard to maintain, but how often will I need to change how the + operation works? 

Furthermore, I actually spent more time trying to determine how to handle Vectors vs Points.  I liked the idea of being able to distinguise based on how they are used, but implementing them twice was aweful. I decided to just implement Vector3, and never use Point.  Variable names of type Vector3 that are used as points can be inferred by their name, such as location.

One consequence of this decision is that I had to implement casting methods for Color and Vector3 so I could convert between them when needed. This mainly comes into play when you want to color a surface based on normals.

```
pub fn ray_color_normals(r: &Ray, world: &impl Hittable) -> Color {
    match world.hit(r, 0.0, INFINITY) {
        None => return ray_color_bg(r),
        Some(hit_record) => {
            return 0.5 * (Color::from_vector(hit_record.normal) + Color::new(1.0,1.0,1.0,));
        },
    }
}
```
