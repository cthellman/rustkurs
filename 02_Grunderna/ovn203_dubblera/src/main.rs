/* Övning 203: Dubblera

Skriv funktionen dubblera som tar en referens till
en isize som parameter och dubblerar värdet på
variabeln som refereras.

 */

fn dubblera(tal: &mut isize) {
    *tal *= 2;
}

fn main() {
    let mut n: isize = 7;
    dubblera(&mut n);
    println!("Värde: {}", n);
}

// Lämna koden nedan oförändrad

#[test]
fn test_dubblera1() {
    let mut i: isize = -7;
    dubblera(&mut i);
    assert_eq!(i, -14);
}

#[test]
fn test_dubblera2() {
    let mut i: isize = 0;
    dubblera(&mut i);
    assert_eq!(i, 0);
}

#[test]
fn test_dubblera3() {
    let mut i: isize = 99;
    dubblera(&mut i);
    assert_eq!(i, 198);
}
