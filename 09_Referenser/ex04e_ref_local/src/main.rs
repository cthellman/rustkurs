/*  ex04e_ref_local  */
fn main() {
    let poäng_ref;
    {
	let mut poäng = 33;
        poäng_ref = &mut poäng;
    }
    println!("Värdet är {}", *poäng_ref);
}
// error[E0597]: `poäng` does not live long enough


