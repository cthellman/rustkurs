/*  ex02a_hej_hopp  */
fn hej(n: usize) -> Option<usize> {
    if n > 5 { Some(n) } else { None }
}

fn hopp(a: usize, b: usize) -> Option<usize> {
    Some(hej(a)? + hej(b)?)
}

fn main() {
    println!("Summa: {:?}", hopp(3, 9));
    println!("Summa: {:?}", hopp(7, 8));
}
// Summa: None
// Summa: Some(15)





