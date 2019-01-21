# Mandelbrot set viewer

This tool plots the Mandelbrot set for a defined area. It outputs the result as a `png` file.

![example](https://github.com/agherzan/mandelbrot-viewer/raw/master/images/mandelbrot.png)

For more info and runtime arguments run:
```sh
mandelbrot-viewer -h

mandelbrot-viewer 0.0.1
Andrei Gherzan <andrei@gherzan.ro>


USAGE:
    mandelbrot-viewer [FLAGS] --first-point <first_point> --last-point <last_point> --output <FILE> --resolution <resolution>

FLAGS:
    -h, --help        Prints help information
    -p, --parallel    Run the program on multiple threads
    -V, --version     Prints version information

OPTIONS:
    -f, --first-point <first_point>    Top left point on the cartesian graph [default: -2,1]
    -l, --last-point <last_point>      Bootom right point on the cartesian graph [default: 1,-1]
    -o, --output <FILE>                Filename for the output png [default: mandelbrot.png]
    -r, --resolution <resolution>      Image resolution of the output [default: 900x600]
```

This implementation is inspired from `Programming Rust: Fast, Safe Systems Development 1st Edition` by Jim Blandy and Jason Orendorff.
