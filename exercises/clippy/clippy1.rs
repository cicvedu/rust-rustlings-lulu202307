// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::f32;

fn main() {
    let pi: f32 = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.pow(2);

    println!("The area of a circle with radius {:.2} is {:.2}!", radius, area)
}
