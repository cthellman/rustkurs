/*  ex17f_add  */
// https://doc.rust-lang.org/std/ops/trait.Add.html

trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}





