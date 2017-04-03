//! sierpinsky triangle image generator

extern crate image;
extern crate rand;

use rand::Rng;
use std::fs::File;
use std::path::Path;
use std::fmt;

/// Image dimensions
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const ITERATIONS: u32 = 100;

/// points used for building triangle and plot on canvas
pub struct Point{
    x: f64,
    y: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}


/// Halfway function
pub fn halfway_points(pt1: &Point, pt2: &Point) -> Point {
    Point { 
        x: (pt1.x + pt2.x) / 2.0,
        y: (pt1.y + pt2.y) / 2.0
    }

}

/// Scale Point on plane from 2..-2
pub fn scale_point(pt: &Point) -> Point {
    let r = 4.0;
    Point {
        x: ((pt.x * r / HEIGHT as f64) - r / 2.0),
        y: ((pt.y * r / WIDTH as f64) - r / 2.0) * -1.0
    }
}


/// Mandlebrot
/// f(z) = z**2 + c
pub fn mandlebrot(pt: Point) -> bool {
    let cr = pt.x;
    let ci = pt.y;
    let mut zr = cr;
    let mut zi = ci;

    // The number os iterations acts as a
    // "resolution"

    for i in 0..ITERATIONS {
        if zr.powi(2) + zi.powi(2) > 4.0 {
            return false;
        }

        let new_zr = (zr * zr) - (zi * zi) + cr;
        let new_zi = ((zr * zi) * 2.0) + ci;
        zr = new_zr;
        zi = new_zi;
    }
    true
}

/// main program
pub fn main() {

    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        let pt = Point { x: x as f64, y: y as f64 };
        if mandlebrot(scale_point(&pt)) {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    // let new_point = Point { x: 0.0, y: 0.0 };
    // println!("Point: {}", &new_point);
    // println!("Scaled Point: {}", scale_point(&new_point));
    // println!("Mandlebrot?: {}", mandlebrot(scale_point(&new_point)));

    // for num in 0..100 {
    //     let new_point = Point { x: num as f64, y: 0.0 };
    //     println!("Point: {}", &new_point);
    //     println!("Scaled Point: {}", scale_point(&new_point));
    //     println!("Mandlebrot?: {}", mandlebrot(scale_point(&new_point)));
    // }


    // Loop through canvas and check if the index is part
    // of the mandlebrot set

    // for i in 0..ITERATIONS {
    //     num = rand::thread_rng().gen_range(0,255);
    //     img.put_pixel(pen.x, pen.y, pixel);
    // }

    // let pts: [Point; 3] = [
    //         Point { x: WIDTH / 2, y: 0 },
    //         Point { x: 0, y: HEIGHT },
    //         Point { x: WIDTH, y: HEIGHT }
    // ];

    // let mut num: usize;
    // let mut pen = Point { x: 350, y:350 };
    // let pixel = img[(0, 0)];

    // while counter > 0 {
    //     counter -= 1;
    //     num = rand::thread_rng().gen_range(0, 3);
    //     pen = halfway_points(&pen, &pts[num]);
    //     //println!("Point: {}", pen);
    //     img.put_pixel(pen.x, pen.y, pixel);
    // }

    let ref mut f_out = File::create(&Path::new("mandlebrot.png")).unwrap();
    let _ = image::ImageLuma8(img).save(f_out, image::PNG);
}
