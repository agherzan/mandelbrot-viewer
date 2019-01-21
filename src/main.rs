extern crate clap;
extern crate image;
extern crate num;

mod args;
mod writers;
mod mandelbrot;
mod math;
mod parser;

fn main() {
    let cli_args = args::get_cli();

    let area = parser::parse_pair::<usize>(cli_args.value_of("resolution").unwrap(), 'x')
        .expect("Error parsing image dimensions");
    let first_point = parser::parse_complex(cli_args.value_of("first_point").unwrap()).
        expect("Error parsing the first point.");
    let last_point = parser::parse_complex(cli_args.value_of("last_point").unwrap())
        .expect("Error parsing the last point.");
    let mut pixels = vec![0; area.0 * area.1];

    if cli_args.is_present("parallel") {
        println!("[WARN] Parallel mode not yet implemented.");
    }
    println!("[INFO] Plotting...");
    mandelbrot::plot_set(&mut pixels, area, first_point, last_point);
    writers::export_image(cli_args.value_of("output").unwrap(), &pixels, area)
        .expect("Error exporting the png file.");
    println!("[INFO] Done. See {} file.", cli_args.value_of("output").unwrap());
}
