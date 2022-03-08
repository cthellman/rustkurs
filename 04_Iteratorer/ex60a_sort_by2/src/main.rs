/*  ex60a_sort_by2  */
fn sss(lista: &mut [usize]) {
    lista.sort_by(|a, b|
	(a%10).cmp(&(b%10)).then(a.cmp(b))
    );
}

fn main() {
    let mut lista = [12, 19, 22, 33, 27, 13];
    sss(&mut lista);
    println!("{:?}", lista);
}
// [12, 22, 13, 33, 27, 19]

