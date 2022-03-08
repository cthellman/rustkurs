/*  ex34a_referens  */
fn main() {
    let poäng = 33;
    let referens = &poäng;
    let ny_poäng = *referens + 10;
    println!("Värdet är {} var {}", ny_poäng, poäng);
}
// Värdet är 43 var 33
