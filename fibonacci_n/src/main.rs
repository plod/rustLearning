extern crate clap;

use clap::{Arg, App};

// learnt this from: 
// https://stackoverflow.com/questions/27840394/how-can-a-rust-program-access-metadata-from-its-cargo-package

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    //learned about clap from https://mattgathu.github.io/writing-cli-app-rust/
    let matches = App::new("fibonacci_m")
        .version(VERSION)
        .author(AUTHORS)
        .about("Fibonacci Calculator written in Rust")
        .arg(Arg::with_name("n")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("number of fibonacci number to calculate"))
        .get_matches();
    let n = matches.value_of("n").unwrap();
    let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("n is NaN"),
        };

    let val = calc_fib(n);
    println!("The fibonacci number at {} is {}:", n, val);
}

//using binet's formula https://artofproblemsolving.com/wiki/index.php?title=Binet%27s_Formula
fn calc_fib(x: u32) -> u64 {
    let a0r0n = ((1 as f64) + (5 as f64).sqrt()).powf(x as f64);
    let a1r1n = ((1 as f64) - (5 as f64).sqrt()).powf(x as f64);
    let delim = (5 as f64).sqrt() * (2 as f64).powf(x as f64);
    //println!("a0r0n: {}, a1r1n: {}, delim: {}", a0r0n, a1r1n, delim); //for debug
    let answer = (a0r0n - a1r1n)/delim;
    //println!("answer: {}", answer); //for debug
    answer as u64
}