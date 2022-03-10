/*  ex35a_mut_ref  */
fn main() {
    let mut poäng = 33;
    {
        let r = &mut poäng; // Write rights to the variable (the reference)
        *r += 10;           // Change the variable value that the reference point at     
    }                       // the r variable dies here
    println!("Värdet är {}", poäng);
}
// Värdet är 43
