/*  ex07f_read  */

// https://doc.rust-lang.org/std/io/trait.Read.html
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    // ...
}





