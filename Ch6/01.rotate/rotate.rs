use std::io::{stdin, Read};
fn main() {
    for mut c in stdin().bytes().map(|x| x.expect("cannot read from stdin")) {
        if c == 'z' as u32 as u8 {
            c = 'a' as u32 as u8;
        } else if char::from(c).is_ascii_lowercase() {
            c += 1;
        }
        print!("{}", char::from(c));
    }
}
