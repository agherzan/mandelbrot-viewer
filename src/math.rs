use num::Complex;

// Converts a pixel to a point in cartesian graph
pub fn pixel_to_complex(pixel: (usize, usize),
                    area: (usize, usize),
                    first_point: Complex<f64>,
                    last_point: Complex<f64>)
    -> Complex<f64>
{
    Complex {
        re: first_point.re + pixel.0 as f64 * (last_point.re - first_point.re) / area.0 as f64,
        im: first_point.im - pixel.1 as f64 * (first_point.im - last_point.im) / area.1 as f64
    }
    // If you are confused just think of the units (you need to get rid of pixels aka pxl/pxl)
}

#[test]
fn test_pixel_to_complex() {
    assert_eq!(pixel_to_complex((1,1), (10,10),
        Complex { re: 0.0, im: 0.0 },
        Complex { re: 10.0, im: -10.0 }),
        Complex { re: 1.0, im: -1.0 });
    assert_eq!(pixel_to_complex((0,0), (10,10),
        Complex { re: 0.0, im: 0.0 },
        Complex { re: 10.0, im: -10.0 }),
        Complex { re: 0.0, im: -0.0 });
    assert_eq!(pixel_to_complex((10,10), (10,10),
        Complex { re: 0.0, im: 0.0 },
        Complex { re: 10.0, im: -10.0 }),
        Complex { re: 10.0, im: -10.0 });
}

