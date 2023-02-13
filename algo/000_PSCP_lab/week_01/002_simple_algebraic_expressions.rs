/*

Write a program evaluate the following algebraic expressions after
reading necessary values as command line args.

i)(ax+b)/(ax–b)
ii)2.5logx+Cos32+|x2 +y2|+v2xy
iii)x5+10x4 +8,x3+4x+2
iv)aekt

*/

use std::collections::HashMap;

fn main() {
    let a: f64 = 10_f64;
    let b: f64 = 26_f64;
    let x: f64 = 2_f64;
    let y: f64 = 3_f64;
    let v: f64 = 4_f64;
    let mut expressions_map: HashMap<String, f64> = HashMap::new();
    expressions_map.insert(
        "((a * x) + b) / ((a * x) - b)".to_string(),
        ((a * x) + b) / ((a * x) - b),
    );
    expressions_map.insert(
        "2.5 * log2(x) + cos(32) + | x^2 + y^2 | + v * 2 * x * y".to_string(),
        2.5_f64 * x.log2() + 32_f64.cos() + (x.powi(2) + y.powi(2)).abs() + v * x * y,
    );
    for (key, value) in &expressions_map {
        println!("{key} = {value}");
    }
}
