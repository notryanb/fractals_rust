//! sierpinsky triangle image generator

extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;

/// points used for building triangle and plot on canvas
pub struct Point{
    x: u32,
    y: u32
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

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
        counter = counter - 1;
        num = rand::thread_rng().gen_range(0, 3);
        pen.x = (pen.x + pts[num].x) / 2;
        pen.y = (pen.y + pts[num].y) / 2;
        img.put_pixel(pen.x, pen.y, pixel);
    }

    let ref mut f_out = File::create(&Path::new("tri.png")).unwrap();
    let _ = image::ImageLuma8(img).save(f_out, image::PNG);
}
