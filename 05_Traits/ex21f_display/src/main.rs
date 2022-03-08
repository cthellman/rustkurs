/*  ex21f_display  */
// https://doc.rust-lang.org/std/fmt/trait.Display.html

pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
// även: to_string


