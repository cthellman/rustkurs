/*  ex12a_funktion  */
fn addera(a: isize, b: isize) -> isize {
    let c = a + b;  // -> <datatype of the return value>
    c               // return value, without semicolon
}

fn main() {
    let a = 5;
    let b = 3;
    println!("{a} + {b} = {}", addera(a, b));
}
// 5 + 3 = 8

/*

Ett kodblock, dvs. kod inom klamrar ("måsvingar"), kan ha ett resultat i
Rust. Resultatet är det sista värdet som beräknas före den avslutande
klammern. Obs! Om man har ett semikolon före den avslutande klammern,
så räknas värdet som "tomt", vilket skrivs som det tomma parentesparet ()
i Rust.

I en funktion får man göra explicita return om man vill, men man bör
inte göra det i sista raden. T.ex. ger cargo clippy varningen

   warning: unneeded `return` statement

i så fall.

I funktionen addera hade man förstås inte behövt använda variabeln c.
Det hade räckt att returnera a+b.

*/
