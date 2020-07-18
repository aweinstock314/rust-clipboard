use crossclip::{Clipboard, ClipboardError, SystemClipboard};

fn main() -> Result<(), ClipboardError> {
    let clipboard = SystemClipboard::new()?;
    clipboard.set_string_contents(String::from("Hello, world!"))?;
    println!(
        "Cliboard contents set to `{}`",
        clipboard.get_string_contents()?
    );
    Ok(())
}
