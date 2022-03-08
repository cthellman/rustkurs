/*  ex03a_eq  */
fn main() {
    let x: f64 = 2.5;
    let y: f64 = f64::NAN;
    println!("x == x: {}", x.eq(&x));
    println!("y == y: {}", y.eq(&y));
    println!("{} is NaN: {}", y, y.is_nan());
}
// x == x: true
// y == y: false
// NaN is NaN: true






