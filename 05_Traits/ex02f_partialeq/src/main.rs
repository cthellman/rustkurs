/*  ex02f_partialeq */
// https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

pub trait PartialEq<Rhs = Self>
where Rhs: ?Sized, {

    fn eq(&self, other: &Rhs) -> bool;

    fn ne(&self, other: &Rhs) -> bool { ... }
}





