/* Övning 602: kopiera

Skriv funktionen kopiera som tar
två filnamn som argument. Alla rader
från första filen som är på formatet
Bill 12
(dvs. en sträng följd av blanktecken
följd av ett icke-negativt heltal)
ska skrivas till den andra filen.

 */

use std::io::{BufRead, BufReader, Write};


fn main() {
    kopiera("scores.txt", "kopia.txt").unwrap();
}
