/* Övning 702: ordfrekvens

Skapa funktionen frekvens. Den ska ta ett filnamn som
parameter. Funktionen ska räkna antalet förekomster
av varje ord i filen.

Funktionen ska returnera en option med en tupel av
typen (String, usize), t.ex. Some(("och", 14)), med
det mest frekventa ordet samt antalet förekomster.

Gör ingen skillnad mellan små och stora bokstäver
(använd metoden to_lowercase).

Ifall flera ord förekommer lika många gånger, så välj
något av dem.

Vid fel, eller om inga ord finns, returnera None.

*/



fn main() {
    println!("Ord: {:?}", frekvens("ordfil.txt"));
}




