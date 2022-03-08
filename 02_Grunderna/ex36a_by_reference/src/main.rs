/*  ex36a_by_reference  */
fn öka(tal: &mut i32) {
    *tal += 10;
}

fn main() {
    let mut poäng = 33;
    öka(&mut poäng);
    println!("Värdet är {}", poäng);
}
// Värdet är 43
