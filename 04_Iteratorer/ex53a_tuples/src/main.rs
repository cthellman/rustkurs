/*  ex53a_tuples  */
fn main() {
    let mut v = [(3,1), (0,7), (3,2),
		 (2,5), (1,0), (2,0)];
    v.sort();
    println!("{:?} ", v);
}
// [(0, 7), (1, 0), (2, 0), (2, 5), (3, 1), (3, 2)] 
