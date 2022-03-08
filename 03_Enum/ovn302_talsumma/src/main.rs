/* Övning 302:

Kör programmet nedan först kompilerat som debug

   cargo run

och sedan som release:

   cargo run --release

Vad blir skillnaden?
Vad händer om man gör sluttal ännu större?

*/
    
fn talsumma(n: usize) -> usize {
    (1..=n).fold(0, |summa, tal| summa + tal)
}

fn main() {
    let sluttal: usize = 100_000_000;
    println!("Summa: {}", talsumma(sluttal));
}
