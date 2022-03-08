/*  ex13e_move  */

#[derive(PartialEq, Debug)]
struct Punkt { x: f64, y: f64 }

fn main() {
    let p = Punkt { x: 4.0, y: -1.0 };
    let p2 = p;
    println!("Lika: {}", p == p2);
}
// error[E0382]: borrow of moved value

