/// Mandlebrot set: the set of complex numbers `c` for which
/// z does NOT fly out to infinity. Where: 
/// `z = z * z + c`
/// 
/// Mandlebrot plot: using c.real and c.imaginary as (x, y) 
/// coordinates on a cartesian plane, we plot all c, where:
/// Black point indicates `c` is in the Mandlebrot set
/// White point indicates that `c` is NOT in the Mandelbrot set

extern crate num;
use num::Complex; //for complex number structs
fn main() {
    println!("Hello, world!");
}

fn complex_square_add_loop(c: Complex<f64>){
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}