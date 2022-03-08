/* Övning 202: Talsumma

Skriv funktionen talsumma som tar ett usize
som parameter och returnerar summan av alla tal
från 0 till och med det givna talet.
T.ex. ska talsumma(3) returnera 6.

När det går att kompilera och exekvera programmet,
kör enhetstesterna, t.ex. med kommandot

   cargo test

eller via menyerna i editorn.

 */



fn main() {
    println!("Summa: {}", talsumma(20));
}

// Lämna koden nedan oförändrad

#[test]
fn test_talsumma1() {
    assert_eq!(talsumma(1), 1);
}

#[test]
fn test_talsumma10() {
    assert_eq!(talsumma(10), 55);
    assert_eq!(talsumma(20), 210);
}

#[test]
fn test_talsumma0() {
    assert_eq!(talsumma(0), 0);
}
