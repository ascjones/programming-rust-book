extern crate num;
use num::Complex;

fn main() {
    println!("Hello, world!");
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escapes(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    return None;
}

/// Return the point on the complex plane corresponding to a given pixel in the
/// bitmap.
///
/// `bounds` is a pair giving the width and height of the bitmap. `pixel` is a
/// pair indicating a particular pixel in that bitmap. The `upper_left` and
/// `lower_right` parameters are points on the complex plane designating the
/// area our bitmap covers.
fn pixel_to_point(bounds: (usize, usize), 
                  pixel: (usize, usize),
                  upper_left: (f64, f64),
                  lower_right: (f64, f64))
    -> (f64, f64) 
{    
    // It might be nicer to find the position of the *middle* of the pixel,
    // instead of its upper left corner, but this is easier to write tests for.
    let (width, height) = (lower_right.0 - upper_left.0,
                           upper_left.1 - lower_right.1);
    (upper_left.0 + pixel.0 as f64 * width / bounds.0 as f64,
     upper_left.1 - pixel.1 as f64 * height / bounds.1 as f64)
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,100), (25, 75), 
                              (-1.0, 1.0), (1.0, -1.0)), 
        (-0.5, -0.5));
}
