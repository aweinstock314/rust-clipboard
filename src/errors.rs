use std::fmt::{self, Display, Formatter};

#[cfg(any(target_os="linux", target_os="openbsd"))]
use x11_clipboard::error::Error as X11Error;

use std::string::FromUtf8Error;
use std::error::Error;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum ClipboardError {
    Unimplemented,
    IoError(IoError),
    EncodingError(FromUtf8Error),
    #[cfg(any(target_os = "linux", target_os="openbsd"))]
    X11ClipboardError(X11Error),
    #[cfg(target_os = "macos")]
    MacOsClipboardError(MacOsError),
    #[cfg(target_os = "windows")]
    WindowsClipboardError(WinError),
}

#[cfg(target_os="windows")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WinError {
    EmptyClipboard,
    FormatNoSize,
}

#[cfg(target_os = "windows")]
impl Display for WinError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(target_os = "windows")]
impl Error for WinError {
    fn description(&self) -> &str {
        use self::WinError::*;
        match *self {
            EmptyClipboard => "Empty clipboard or couldn't determine format of clipboard contents",
            FormatNoSize => "Could not determine the length of the clipboard contents"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<IoError> for ClipboardError {
    fn from(e: IoError) -> Self {
        ClipboardError::IoError(e)
    }
}

#[cfg(target_os="windows")]
impl From<WinError> for ClipboardError {
    fn from(e: WinError) -> Self {
        ClipboardError::WindowsClipboardError(e)
    }
}

#[cfg(any(target_os="linux", target_os="openbsd"))]
impl From<X11Error> for ClipboardError {
    fn from(e: X11Error) -> Self {
        ClipboardError::X11ClipboardError(e)
    }
}

#[cfg(target_os="macos")]
impl From<MacOsError> for ClipboardError {
    fn from(e: MacOsError) -> Self {
        ClipboardError::MacOsClipboardError(e)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg(target_os = "macos")]
pub enum MacOsError {
    PasteWriteObjectsError,
    ReadObjectsForClassesEmpty,
    ReadObjectsForClassesNull,
    PasteboardNotFound,
    NullPasteboard,
}

#[cfg(target_os = "macos")]
impl Display for MacOsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::MacOsError::*;
        let msg = match *self {
            PasteWriteObjectsError => "Could not paste objects to clipboard",
            ReadObjectsForClassesEmpty => "Clipboard is empty",
            ReadObjectsForClassesNull => "No objects to read",
            PasteboardNotFound => "Pasteboard not found",
            NullPasteboard => "General pasteboard not found",
        };
        write!(f, "{}", msg)
    }
}

impl Display for ClipboardError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::ClipboardError::*;
        match self {
            Unimplemented => write!(f, "Clipboard::Unimplemented: Attempted to get or set the clipboard, which hasn't been implemented yet."),
            IoError(ref e) => write!(f, "Clipboard::IoError: {} cause: {:?}", e.description(), e.cause()),
            EncodingError(ref e) => write!(f, "Clipboard::EncodingError: {} cause: {:?}", e.description(), e.cause()),
            #[cfg(any(target_os="linux", target_os="openbsd"))]
            X11ClipboardError(ref e) => write!(f, "X11ClipboardError: {}", e),
            #[cfg(target_os="macos")]
            MacOsClipboardError(ref e) => write!(f, "MacOsClipboardError: {}", e),
            #[cfg(target_os="windows")]
            WindowsClipboardError(ref e) => write!(f, "WindowsClipboardError: {} cause: {:?}", e.description(), e.cause()),
        }
    }
}

impl From<FromUtf8Error> for ClipboardError {
    fn from(e: FromUtf8Error) -> Self {
        ClipboardError::EncodingError(e)
    }
}
