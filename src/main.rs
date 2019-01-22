extern crate clap;
extern crate crossbeam;
extern crate image;
extern crate num;
extern crate num_cpus;

mod args;
mod writers;
mod mandelbrot;
mod math;
mod parser;

use num::Complex;
use std::str::FromStr;

fn main() {
    let mut threads = num_cpus::get();
    let threads_str = &threads.to_string();
    let cli_args = args::get_cli(threads_str);

    let area = parser::parse_pair::<usize>(cli_args.value_of("resolution").unwrap(), 'x')
        .expect("Error parsing image dimensions");
    let first_point = Complex::from_str(cli_args.value_of("first_point").unwrap())
        .expect("Error parsing the first point.");
    let last_point = Complex::from_str(cli_args.value_of("last_point").unwrap())
        .expect("Error parsing the last point.");
    let mut pixels = vec![0; area.0 * area.1];

    if cli_args.is_present("threads") {
        threads = parser::parse(cli_args.value_of("threads").unwrap()).expect("Unparsable threads argument.");
    }

    let rows_per_band = area.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * area.0).collect();
        println!("[INFO] Plotting...");
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / area.0;
                let band_area = (area.0, height);
                let band_first_point = math::pixel_to_complex((0, top), area, first_point, last_point);
                let band_last_point = math::pixel_to_complex((area.0, height + top), area, first_point, last_point);
                spawner.spawn(move |_| {
                    mandelbrot::plot_set(band, band_area, band_first_point, band_last_point);
                });
            }
        }).expect("Threading error.");
    }
    writers::export_image(cli_args.value_of("output").unwrap(), &pixels, area)
        .expect("Error exporting the png file.");
    println!("[INFO] Done. See {} file.", cli_args.value_of("output").unwrap());
}
