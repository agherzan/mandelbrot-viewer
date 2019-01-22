use clap::{Arg,App,ArgMatches};

pub fn get_cli(threads: &str) -> ArgMatches {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .value_name("FILE")
             .help("Filename for the output png")
             .default_value("mandelbrot.png")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("resolution")
             .short("r")
             .long("resolution")
             .help("Image resolution of the output")
             .default_value("900x600")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("first_point")
             .short("f")
             .long("first-point")
             .help("Top left point on the cartesian graph")
             .allow_hyphen_values(true)
             .default_value("-2,1")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("last_point")
             .short("l")
             .long("last-point")
             .help("Bootom right point on the cartesian graph")
             .allow_hyphen_values(true)
             .default_value("1,-1")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("threads")
             .short("t")
             .long("threads")
             .help("Number of threads used to compute the set")
             .default_value(threads)
             .takes_value(true))
        .get_matches();
    matches
}
