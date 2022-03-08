/*  ex12a_derive  */
#[derive(PartialEq, Debug)]
struct Punkt {
    x: f64,
    y: f64,
}

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    println!("{:?} Lika: {}", p, p == p);
}
// Punkt { x: 4.0, y: -1.0 } Lika: true





