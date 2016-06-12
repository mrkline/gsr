extern crate getopts;
use getopts::Options;
use std::env;
use std::process::*;

fn print_usage(opts: &Options, code: i32) {
    println!("{}", opts.usage("Usage: gsr [options] <file>"));
    exit(code);
}

enum RenderMode {
    Alpha,
    Color,
    Grayscale
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("a", "alpha", "Render with a transparent background");
    opts.optflag("g", "grayscale", "Render in grayscale");
    opts.optopt("o", "", "Set output file name", "NAME");
    opts.optopt("p", "pages", "Pages to rasterize", "PAGES");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f) }
    };

    if matches.opt_present("h") {
        print_usage(&opts, 0);
    }

    let alpha_present = matches.opt_present("a");
    let grayscale_present = matches.opt_present("g");

    if alpha_present && grayscale_present {
        println!("Both --alpha and --grayscale given. Pick one.");
        exit(1);
    }

    let mode = if alpha_present { RenderMode::Alpha }
        else if grayscale_present { RenderMode::Grayscale }
        else { RenderMode::Color };

    let input = &matches.free;
    match input.len() {
        0 => { println!("No input file provided"); print_usage(&opts, 1) },
        1 => (),
        _ => { println!("Can only take one input file at once"); print_usage(&opts, 1) }
    };
    let input = &input[0];

    let output = "-sOutputFile=".to_string() +
        &matches.opt_str("o").unwrap_or(input.clone() + &".png".to_string());

    let pages = if matches.opt_present("p") {
            "-sPageList=".to_string() + &matches.opt_str("p").unwrap()
        } else {
            String::new()
        };

    // Quiet, batch processing mode, hinting and anit-aliasing maxed.
    let stock_args = ["-dQUIET", "-dSAFER", "-dBATCH", "-dNOPAUSE",
                     "-dGridFitTT=2",  "-dGraphicsAlphaBits=4",
                     "-dTextAlphaBits=4"];

    let mut gs_command = Command::new("echo");

    gs_command
        .args(&stock_args)
        .arg(match mode {
            RenderMode::Alpha => "pngalpha",
            RenderMode::Color => "png16m",
            RenderMode::Grayscale => "pnggray",
            })
        .arg(&output)
        .arg(&input);

    if !pages.is_empty() {
        gs_command.arg(&pages);
    }

    gs_command.status().expect("gs failed");
}
