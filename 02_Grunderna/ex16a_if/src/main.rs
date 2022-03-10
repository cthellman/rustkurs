/*  ex16a_if  */
fn main() {
    let x: f64 = 1.1;

    let a = 5;
    let b = 6;
    let c;

    c = if a > b {a} else {b};  // This is the syntax instead of ?
    println!("C is {c}");

    if x > 1.0 {
	    println!("Stort tal.");
    }
}
// Stort tal.
