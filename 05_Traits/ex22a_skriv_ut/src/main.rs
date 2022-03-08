/*  ex22a_skriv_ut  */
fn skriv_ut(sekvens: &[&str]) {
    for värde in sekvens {
	print!("{:?} ", värde);
    }
    println!();
}

fn main() {
    let v = vec!["Bill", "Steve", "Linus"];
    skriv_ut(&v);
}
// "Bill" "Steve" "Linus"

