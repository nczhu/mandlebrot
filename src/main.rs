/// Mandlebrot set: the set of complex numbers `c` for which
/// z does NOT fly out to infinity. Where: 
/// `z = z * z + c`
/// 
/// Mandlebrot plot: using c.real and c.imaginary as (x, y) 
/// coordinates on a cartesian plane, we plot all c, where:
/// Black point indicates `c` is in the Mandlebrot set
/// White point indicates that `c` is NOT in the Mandelbrot set

extern crate num;
use num::Complex;       // Struct: complex number
use std::str::FromStr;  // Trait: parses value from string

fn main() {
    println!("Hello, world!");
}

fn complex_square_add_loop(c: Complex<f64>){
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/// Takes inputs like: "400x600", "1.0, 0.5".
/// Parses the pair into a Complex number.
/// Returns Some<(x, y)> or None depending on input validity.
/// 
/// Parse_pair is a generic function, usable over any generic type T
/// That implements the FromStr trait, i.e. f64 
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None, 
        Some(index) => {
            //     x coordinate              y coordinate
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)), // Some(tuple)
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    
}