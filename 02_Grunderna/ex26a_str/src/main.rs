/*  ex26a_str  */
fn main() {
    let s1 = "Hej\thopp";
    let s2 = r"Tecknen \n ger radbrytning";
    println!("{}", s1);
    println!("{}", s2);
    println!("{:?}", b"G\xf6ran");
}
// Hej	hopp
// Tecknen \n ger radbrytning
// [71, 246, 114, 97, 110]

/*

Explicita strängar ("string literals") skrivs inom citationstecken.
Tecknet \ har specialbetydelse; \t betyder TAB-tecken, \n betyder
radmatning, \\ betyder bakvänt snedstreck osv.

Ifall man skriver ett r före citationstecknet, så tas specialbetydelsen
av \ bort.

Observera! I Rust är strängar kodade i utf-8! Källkoden ska använda utf-8
som teckenkodning (eller hålla sig till ASCII).

Om man har text kodad i någon annan teckenkodning, så måste man betrakta
det som "binärdata", vilket i Rust betyder "en följd av bytes".

Vill man skriva en explicit "binärsträng" i Rust, så ska man sätta
tecknet r före citationstecknet, och icke-ASCII-tecken skrivs med
hexadecimala koder.

Formateringskoden :? betyder att man vill göra en "debug-utskrift" av
motsvarande värde.

*/
