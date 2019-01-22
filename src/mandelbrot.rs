use math::pixel_to_complex;
use num::Complex;

// Check if parameter c is part of the Mandelbrot set using at most `iterations` number of
// iterations.
fn check_if_member(c: Complex<f64>, iterations: u32) -> Result<u32,u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..iterations {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Err(i);
        }
    }
    Ok(0)
}

// Plot a mandelbrot set to a buffer array
pub fn plot_set(buffer: &mut[u8],
            area: (usize, usize),
            first_point: Complex<f64>,
            last_point: Complex<f64>)
{
    let iterations = 255;
    for row in 0..area.1 {
        for column in 0..area.0 {
            let point = pixel_to_complex((column, row), area, first_point, last_point);
            buffer[row * area.0 + column] = check_if_member(point, iterations)
                .unwrap_or_else(|count| 255 - count) as u8;
        }
    }
}

#[test]
fn test_check_if_member() {
    let iterations = 255;
    assert_eq!(check_if_member(Complex {re: 1.0, im: 1.0}, iterations), Err(1));
    assert_eq!(check_if_member(Complex {re: 1.0, im: -1.0}, iterations), Err(1));
    assert_eq!(check_if_member(Complex {re: 0.0, im: 0.0}, iterations), Ok(0));
    assert_eq!(check_if_member(Complex {re: -1.0, im: 0.1}, iterations), Ok(0));
    assert_eq!(check_if_member(Complex {re: 2.0, im: 2.0}, iterations), Err(0));
}

