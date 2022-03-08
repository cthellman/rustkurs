/*  ex05e_ret_local  */
fn p() -> &usize {
    let mut poäng = 33;
    &mut poäng
}

fn main() {
    let poäng_ref = p();
    println!("Värdet är {}", *poäng_ref);
}
// error[E0515]: cannot return reference to local variable `poäng`


