/*  ex18a_flush  */
use std::io::Write;

fn main() {
    let mut namn = String::new();
    print!("Vad heter du? ");
    std::io::stdout().flush().expect("Misslyckades");
    std::io::stdin().read_line(&mut namn)
	.expect("Kunde inte läsa från stdin");
    println!("Hej, {}!", namn.trim());
}
// Vad heter du? Göran
// Hej, Göran!
