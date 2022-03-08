/*  ex57a_jmf  */
fn jmf(a: &usize, b: &usize) -> std::cmp::Ordering {
    if a > b { std::cmp::Ordering::Greater }
    else if a == b { std::cmp::Ordering::Equal }
    else { std::cmp::Ordering::Less }
}

fn main() {
    let mut a = [12, 19, 33, 27];
    a.sort_by(jmf);
    println!("{:?}", a);
}
// [12, 19, 27, 33]

