/* Övning 204: Fibonacci

Skriv funktionen fibomax. Den ska ta en u32 som parameter
och ska returnera det största Fibonaccitalet som är mindre
än eller lika med det givna talet.

T.ex. ska fibomax(100) returnera 89.

*/

fn fibomax(tal: u32) {
    let mut a: u32 = 0;
    let mut b = 1;
    while a =< tal {
	println!("{} ", a);
	let c = a + b;
	a = b;
	b = c;
    }
}

fn main() {
    println!("Resultat: {}", fibomax(100));
}

// Lämna koden nedan oförändrad

#[test]
fn test_fibomax1() {
    assert_eq!(fibomax(100), 89);
}

#[test]
fn test_fibomax2() {
    assert_eq!(fibomax(300), 233);
}

#[test]
fn test_fibomax3() {
    assert_eq!(fibomax(0), 0);
}

#[test]
fn test_fibomax4() {
    assert_eq!(fibomax(1), 1);
}

#[test]
fn test_fibomax5() {
    assert_eq!(fibomax(2), 2);
}

#[test]
fn test_fibomax6() {
    assert_eq!(fibomax(3_000_000_000), 2971215073);
}
