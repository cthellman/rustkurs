/*  ex07a_partial_cmp */
fn main() {
    let x: f64 = 1.7;
    let y: f64 = 2.5;
    let z: f64 = f64::NAN;
    println!("x < y: {:?}", x.partial_cmp(&y));
    println!("y < z: {:?}", y.partial_cmp(&z));
}
// x < y: Some(Less)
// y < z: None





