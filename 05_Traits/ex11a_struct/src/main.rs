/*  ex11a_struct  */
struct Punkt {
    x: f64,
    y: f64,
}

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    println!("Punkt: ({}, {})", p.x, p.y);
}
// Punkt: (4, -1)





