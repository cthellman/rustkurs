/*  ex09f_write  */

// https://doc.rust-lang.org/std/io/trait.Write.html
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()> { ... }
    // ...
}





