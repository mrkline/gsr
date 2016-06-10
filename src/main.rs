extern crate getopts;
use getopts::Options;
use std::env;
use std::process::*;

fn print_usage(opts: &Options, code: i32) {
    println!("{}", opts.usage("Usage: gsr [options] <file>"));
    exit(code);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("p", "pages", "pages to rasterize", "PAGES");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f) }
    };

    if matches.opt_present("h") {
        print_usage(&opts, 0);
    }
    let input = &matches.free;
    match input.len() {
        0 => { println!("No input file provided"); print_usage(&opts, 1) },
        1 => (),
        _ => { println!("Can only take one input file at once"); print_usage(&opts, 1) }
    };
    let input = &input[0];

    let output = matches.opt_str("o").unwrap_or("lolout".to_string());

    let mut gs_command = Command::new("echo");
    gs_command.arg(input)
             .arg(output)
             .status().expect("gs failed");
}
