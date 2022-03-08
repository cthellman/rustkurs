/*  ex09a_array  */
fn main() {
    let lista: [usize; 5] = [2, 3, 5, 7, 11];
    println!("{} och {}", lista[0], lista[3]);
    let i = rand::random::<usize>();
    println!("{}", lista[i]); // Trolig panic
}
// 2 och 7
// panic: index out of bounds


