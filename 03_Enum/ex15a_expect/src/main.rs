/*  ex15a_expect  */
fn main() {
    let tal = String::from("21.5");   
    let n = tal.parse::<usize>().expect("Ej tal");
    println!("Talet är {}", n);
}
// Thread 'main' panicked at 'Ej tal


