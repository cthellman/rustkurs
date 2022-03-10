/*  ex18a_unit  */
fn main() {
    let a = 5;
    let b = 3;
    let c = if a > b { a; } else { b; };
    println!("Största värdet: {:?}", c);    // debug-printout :? c is an unknown datatype  
}
// Största värdet: ()
