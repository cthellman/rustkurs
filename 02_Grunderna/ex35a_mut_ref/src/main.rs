/*  ex35a_mut_ref  */
fn main() {
    let mut poäng = 33;
    {
        let r = &mut poäng;
        *r += 10;
    }
    println!("Värdet är {}", poäng);
}
// Värdet är 43
