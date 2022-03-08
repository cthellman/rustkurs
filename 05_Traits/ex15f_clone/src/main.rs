/*  ex15f_clone  */
// https://doc.rust-lang.org/std/clone/trait.Clone.html

pub trait Clone {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { ... }
}

