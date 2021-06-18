# 1 RustRays Overview

![Current Render](/readme-files/image.png)

This is my attempt at following the excellent book by Peter Shirley, [Raytracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html), implemented in Rust.

I am a new Rustacean. In fact, this is my first Rust program. After starting the book without any Rust knowledge, I realized I wanted to implement this in the "Rust" way as much as possible.  Therefore, I took a pause from implementation, read [The Rust Book](https://doc.rust-lang.org/stable/book/), then started again.  

Where I've found a particular change necessary for Rust, or noticed some improvements by implementing in Rust, I note this in comments.

# 2 Output an Image

First up, is generating an image. In his C++ implementation, Peter S. uses static casting to output integer r, g, and b values with a dynamic range 0 - 255.999. In my implementation of a Color structure, I implement the Display trait to output PPM formated color pixel values. I have a DYN_RANGE and MAX_VAL constants to perform the normalization to a 0 - 255.999 range from 0.0 - 1.0.

```rust
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
        write!(f, "{}, {}, {}", (self.r.clamp(0.0, 0.999)*MAX_VAL) as u32, 
                                (self.g.clamp(0.0, 0.999)*MAX_VAL) as u32, 
                                (self.b.clamp(0.0, 0.999)*MAX_VAL) as u32)
    }
}
```

# 3 The Vector3 Class

My first real "Idiomatic Rust" quandary was whether to implement the Vector3 and Color as separate structures or aliased, like the original author. This became apparent as I didn't like Rust's aliasing capabilities (made it seem clunky), and I wanted to get Color operations that made sense for colors, but as methods, rather than utility functions.  I had to implement many operations twice, which can be hard to maintain, but how often will I need to change how the + operation works? 

Furthermore, I actually spent more time trying to determine how to handle Vectors vs Points.  I liked the idea of being able to distinguise based on how they are used, but implementing them twice was aweful. I decided to just implement Vector3, and never use Point.  Variable names of type Vector3 that are used as points can be inferred by their name, such as location.

One consequence of this decision is that I had to implement casting methods for Color and Vector3 so I could convert between them when needed. This mainly comes into play when you want to color a surface based on normals.

```rust
pub fn ray_color_normals(r: &Ray, world: &impl Hittable) -> Color {
    match world.hit(r, 0.0, INFINITY) {
        None => return ray_color_bg(r),
        Some(hit_record) => {
            return 0.5 * (Color::from_vector(hit_record.normal) + Color::new(1.0,1.0,1.0,));
        },
    }
}
```

# 4 Rays, Camera, Background!

This is where I hit my first snag with borrowing and mutability. I originally implemented the ```at(t)``` function as I would with any languange:

```rust
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3
}

impl Ray {
    pub fn at(self, t: f32) -> Point3 {
        self.origin + t*self.direction
    }
}
```

Great! Now I can create rays and find a location at ```t```.  However, this quickly fell apart when using this in the context of a hittable object (implemented as a Hittable trait).

```rust
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        //find root
        //...
        let p = r.at(root);
        let outward_normal = (p - self.center)/self.radius;
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};
        Some(HitRecord {t:root, p:p, normal:normal, front_face: false})
    }
}
```

I pass in the ray as an immutable reference because I don't need to change the ray, but only use it to find where it lands.  However, since my 'at' method has self as a mutable value, issues ensued because here I accept the ray as an immutable reference. There were a few more steps I glossed over, but essentially, I had to make the 'at' method use an immutable reference such that I could call it on borrowed references of Ray.

```rust
impl Ray {
    pub fn at(&self, t: f32) -> Point3 { // had to use &self to make it an unmutable reference to self, so this could be called by borrowers of *self
        self.origin + t*self.direction
    }
}
```

I also learned about using Option enums to avoid the need to have the hit record returned through a argument.  I have to say, this was refreshing at first, then felt a little verbose due to the return value handling, but eventually I came to really like this methodology.  This made for a nice implementation of hittable list.

```rust
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        //Was able to remove "hit_anything" because that logic is encapsulated in the use of Option<>, yay Rust!
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            match hittable.hit(r, t_min, closest_so_far) {
                None => (),
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    temp_rec = Some(hit_record);
                }
            }
        }
        temp_rec
    }
}
```
