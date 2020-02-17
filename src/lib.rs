/*
Copyright 2016 Avraham Weinstock

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

#![crate_name = "clipboard"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#[cfg(all(unix, feature = "x11", not(any(target_os="macos", target_os="android", target_os="emscripten"))))]
extern crate x11_clipboard as x11_clipboard_crate;

#[cfg(all(windows, feature = "windows"))]
extern crate clipboard_win;

#[cfg(all(target_os="macos", feature = "macos"))]
#[macro_use]
extern crate objc;
#[cfg(all(target_os="macos", feature = "macos"))]
extern crate objc_id;
#[cfg(all(target_os="macos", feature = "macos"))]
extern crate objc_foundation;

mod common;
pub use common::ClipboardProvider;

#[cfg(all(unix, feature = "x11", not(any(target_os="macos", target_os="android", target_os="emscripten"))))]
pub mod x11_clipboard;

#[cfg(all(windows, feature = "windows"))]
pub mod windows_clipboard;

#[cfg(all(target_os="macos", feature = "macos"))]
pub mod osx_clipboard;

pub mod nop_clipboard;

#[cfg(all(unix, feature = "x11", not(any(target_os="macos", target_os="android", target_os="emscripten"))))]
pub type ClipboardContext = x11_clipboard::X11ClipboardContext;
#[cfg(all(windows, feature = "windows"))]
pub type ClipboardContext = windows_clipboard::WindowsClipboardContext;
#[cfg(all(target_os="macos", feature = "macos"))]
pub type ClipboardContext = osx_clipboard::OSXClipboardContext;
#[cfg(target_os="android")]
pub type ClipboardContext = nop_clipboard::NopClipboardContext; // TODO: implement AndroidClipboardContext (see #52)
#[cfg(not(any(all(unix, feature = "x11", not(any(target_os="macos", target_os="android", target_os="emscripten"))), all(windows, feature = "windows"), all(target_os="macos", feature = "macos"), target_os="android")))]
pub type ClipboardContext = nop_clipboard::NopClipboardContext;

#[test]
fn test_clipboard() {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents("some string".to_owned()).unwrap();
    assert!(ctx.get_contents().unwrap() == "some string");
}
