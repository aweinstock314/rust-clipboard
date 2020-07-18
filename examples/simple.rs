extern crate clipboard2;

use clipboard2::{Clipboard, SystemClipboard};

fn main() {
	let clipboard = SystemClipboard::new().unwrap();
	clipboard.set_string_contents(String::from("Hello")).unwrap();
	println!("{}", clipboard.get_string_contents().unwrap());
}