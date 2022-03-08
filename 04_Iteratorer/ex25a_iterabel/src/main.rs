/*  ex25a_iterabel  */
fn main() {
    let primtal = [2, 3, 5, 7, 11];
    for tal in primtal {
	print!("{} ", tal);
    }
}
// 2 3 5 7 11

/*

Vi såg förut att man kan loopa över en iterator. Men man kan också loopa över
andra saker, t.ex. över arrayer eller vektorer.

Vad som händer när man loopar över ett objekt är att Rust först "hämtar ut"
en iterator från objektet, och sedan anropas next() på iteratorn tills den
returnerar None.

En iterabel är något man kan loopa över. Mer precist är det ett objekt på vilket
man kan anropa into_iter() och få en iterator i retur.
Ännu mer precist är det ett objekt av en typ som har Trait IntoIterator.

Om man anropar into_iter() på något som redan är en iterator, så returneras
iteratorn själv. Dvs. alla iteratorer är iterabler som man kan loopa över.
Däremot finns iterabler (t.ex. en vektor) som inte är iteratorer.

*/

