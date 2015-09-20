use clipboard_win::{get_clipboard_string, set_clipboard};

use std::error::Error;
use std::ffi::OsStr;

pub struct ClipboardContext;

impl ClipboardContext {
    #[inline]
    pub fn new() -> Result<ClipboardContext, Box<Error>> {
        Ok(ClipboardContext)
    }

    #[inline]
    pub fn get_contents(&self) -> Result<String, Box<Error>> {
        Ok(try!(get_clipboard_string()))
    }

    #[inline]
    pub fn set_contents<T: AsRef<OsStr>>(&mut self, data: T) -> Result<(), Box<Error>> {
        Ok(try!(set_clipboard(data.as_ref())))
    }
}
