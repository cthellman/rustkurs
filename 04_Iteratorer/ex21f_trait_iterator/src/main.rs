/*  ex21f_trait_iterator  */

// https://doc.rust-lang.org/std/iter/trait.Iterator.html

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;   // Option is either Some(<value>) or None

    fn count(self) -> usize { ... }
    // ...
}


