/*  ex18a_sekvens  */
fn visa(lista: &[usize]) {
    println!("Sekvens: {:?}", lista);
}

fn main() {
    let primtal = [2, 3, 5, 7, 11];
    visa(&primtal);
    visa(&primtal[2..]);
}
// Sekvens: [2, 3, 5, 7, 11]
// Sekvens: [5, 7, 11]


