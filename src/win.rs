use {
	Clipboard,
	errors::{ClipboardError, WinError},
	clipboard_metadata::{WinContentType, ClipboardContentType}
};

use clipboard_win::{
	Clipboard as SystemClipboard,
	raw::is_format_avail,
};

pub struct WindowsClipboard { }

impl Clipboard for WindowsClipboard {
	type Output = Self;

	fn new() -> Result<Self::Output, ClipboardError> {
		Ok(WindowsClipboard { })
	}

	fn get_contents(&self)
	-> Result<(Vec<u8>, ClipboardContentType), ClipboardError>
	{
		let clipboard = SystemClipboard::new()?;
		let mut format = WinContentType::Bitmap;
		loop {
			if is_format_avail(format.into()) {
				let format_size = match clipboard.size(format.into()) {
					Some(s) => s,
					None => return Err(WinError::FormatNoSize.into()),
				};
				let mut vec = vec![0; format_size];
				clipboard.get(format.into(), &mut vec)?;
				return Ok((vec, ClipboardContentType::WinContentType(format)));
			} else {
				match format.next() {
					Some(f) => format = f,
					None => return Err(WinError::EmptyClipboard.into()),
				}
			}
		}
	}

	fn get_string_contents(&self)
	-> Result<String, ClipboardError>
	{
		SystemClipboard::new()?.get_string().map_err(|e| e.into())
	}

	fn set_contents(&self, contents: Vec<u8>, format: ClipboardContentType)
	-> Result<(), ClipboardError>
	{
		let win_content_type = match format {
			ClipboardContentType::WinContentType(w) => w,
		};
		SystemClipboard::new()?.set(win_content_type.into(), &contents).map_err(|e| e.into())
	}

	fn set_string_contents(&self, contents: String)
	-> Result<(), ClipboardError>
	{
		SystemClipboard::new()?.set_string(&contents).map_err(|e| e.into())
	}
}