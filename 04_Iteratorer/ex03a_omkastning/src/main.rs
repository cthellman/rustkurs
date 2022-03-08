/*  ex03a_omkastning  */
fn main() {
    let mut t = (12, 19, 33, 27);
    t = (t.3, t.1, 4, t.2);
    println!("{:?}", t);
}
// (27, 19, 4, 33)

