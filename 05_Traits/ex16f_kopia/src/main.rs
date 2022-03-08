/*  ex16f_kopia  */
#[derive(PartialEq, Debug, Copy, Clone)]
struct Punkt { x: f64, y: f64 }

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    let p2 = p;
    println!("Lika: {}", p == p2);
    println!("Kopia: {:?}", p.clone());
}
// Lika: true
// Kopia: Punkt { x: 4.0, y: -1.0 }

