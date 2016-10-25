//! sierpinsky triangle image generator

extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;
use std::fmt;

/// points used for building triangle and plot on canvas
pub struct Point{
    x: u32,
    y: u32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn halfway_points(pt1: &Point, pt2: &Point) -> Point {
    Point { 
        x: (pt1.x + pt2.x) / 2,
        y: (pt1.y + pt2.y) / 2
    }

}

/// main program
pub fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let mut counter: u32 = 1_000_000;

    let pts: [Point; 3] = [
            Point { x: WIDTH / 2, y: 0 },
            Point { x: 0, y: HEIGHT },
            Point { x: WIDTH, y: HEIGHT }
    ];

    let mut num: usize;
    let mut pen = Point { x: 350, y:350 };
    let pixel = img[(0, 0)];

    while counter > 0 {
        counter -= 1;
        num = rand::thread_rng().gen_range(0, 3);
        pen = halfway_points(&pen, &pts[num]);
        println!("Point: {}", pen);
        img.put_pixel(pen.x, pen.y, pixel);
    }

    let ref mut f_out = File::create(&Path::new("tri.png")).unwrap();
    let _ = image::ImageLuma8(img).save(f_out, image::PNG);
}
