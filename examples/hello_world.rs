extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let the_string = "Hello, world!";

    ctx.set_contents(the_string.to_owned()).unwrap();
}
