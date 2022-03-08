/* Övning 402: siffersumma

Skriv funktionen sss. Den ska ta en sekvens av u32 som
parameter och ska sortera sekvensen i växande ordning med
avseende på talens siffersumma.

T.ex. ska [12, 19, 33, 27] sorteras som [12, 33, 27, 19]
eftersom siffersummorna är 1+2, 3+3, 2+7 respektive 1+9.

Extrauppgift: Tal som har samma siffersumma ska sorteras
i växande ordning med avseende på talet självt.

T.ex. ska [1234, 56, 28, 82] sorteras som
[28, 82, 1234, 56].


*/


fn main() {
    let mut lista = [1234, 56, 28, 82];
    sss(&mut lista);
    println!("Lista: {:?}", lista);
}

#[test]
fn test_array() {
    let mut lista = [12, 19, 33, 27];
    sss(&mut lista);
    assert_eq!(lista, [12, 33, 27, 19]);
}

#[test]
fn test_vektor() {
    let mut v = vec![9, 0, 88, 345, 15, 124];
    sss(&mut v);
    assert_eq!(v, [0, 15, 124, 9, 345, 88]);
}

#[test]
fn test_tom_lista() {
    let mut lista = [];
    sss(&mut lista);
    assert_eq!(lista, []);
}

#[test]
fn test_lika() {
    let mut lista = [1234, 56, 28, 82];
    sss(&mut lista);
    assert_eq!(lista, [28, 82, 1234, 56]);
}
