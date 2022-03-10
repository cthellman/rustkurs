/*  ex07a_overflow  */
#[allow(arithmetic_overflow)]
fn main() {
    let a: usize = 5;
    let b = 3;
    let c = a + b - 10;
    // let c: isize = (a + b - 10).try_into().unwrap();
    println!("Resultat: {}", c);
}
// thread 'main' panicked at 'attempt to subtract with overflow'
