/*  ex01a_nan  */
fn main() {
    let x: f64 = 2.5;
    let y: f64 = f64::NAN;
    println!("x == x: {:6} x != y: {}", x==x, x!=x);
    println!("y == y: {:6} y != y: {}", y==y, y!=y);
}
// x == x: true   x != y: false
// y == y: false  y != y: true






