/*
Copyright 2019 Gregory Meyer

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

use common::*;
use failure::Fail;
use std::{
    error::Error,
    io::{self, Read},
};
use wl_clipboard_rs::{
    copy::{self, Options, ServeRequests},
    paste, ClipboardType,
};

pub struct WaylandClipboardContext {
    supports_primary_selection: bool,
}

impl ClipboardProvider for WaylandClipboardContext {
    fn new() -> Result<WaylandClipboardContext, Box<dyn Error>> {
        if let Err(e) = paste::get_contents(
            ClipboardType::Primary,
            paste::Seat::Unspecified,
            paste::MimeType::Any,
        ) {
            match e {
                paste::Error::NoSeats | paste::Error::ClipboardEmpty | paste::Error::NoMimeType => {
                    Ok(WaylandClipboardContext {
                        supports_primary_selection: true,
                    })
                }
                paste::Error::PrimarySelectionUnsupported => Ok(WaylandClipboardContext {
                    supports_primary_selection: false,
                }),
                _ => Err(Box::new(e.compat())),
            }
        } else {
            Ok(WaylandClipboardContext {
                supports_primary_selection: true,
            })
        }
    }

    fn get_contents(&mut self) -> Result<String, Box<dyn Error>> {
        if self.supports_primary_selection {
            if let Ok((mut reader, _)) = paste::get_contents(
                ClipboardType::Primary,
                paste::Seat::Unspecified,
                paste::MimeType::Text,
            ) {
                // strange, but rustc won't convert Box<io::Error> into Box<dyn Error> implicitly
                return Ok(read_into_string(&mut reader).map_err(Box::new)?);
            }
        }

        let mut reader = paste::get_contents(
            ClipboardType::Regular,
            paste::Seat::Unspecified,
            paste::MimeType::Text,
        )
        .map_err(into_boxed_error)?
        .0;

        Ok(read_into_string(&mut reader).map_err(Box::new)?)
    }

    fn set_contents(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        let mut options = Options::new();

        options
            .seat(copy::Seat::All)
            .trim_newline(false)
            .foreground(false)
            .serve_requests(ServeRequests::Unlimited);

        if self.supports_primary_selection {
            options
                .clipboard(ClipboardType::Primary)
                .copy(copy::Source::Bytes(data.as_bytes()), copy::MimeType::Text)
                .map_err(into_boxed_error)?;
        }

        options
            .clipboard(ClipboardType::Regular)
            .copy(copy::Source::Bytes(data.as_bytes()), copy::MimeType::Text)
            .map_err(into_boxed_error)
    }
}

fn into_boxed_error<F: 'static + Fail>(fail: F) -> Box<dyn Error> {
    Box::new(fail.compat())
}

fn read_into_string<R: Read>(reader: &mut R) -> io::Result<String> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn wayland_test() {
        let mut clipboard =
            WaylandClipboardContext::new().expect("couldn't create a Wayland clipboard");

        clipboard
            .set_contents("foo bar baz".to_string())
            .expect("couldn't set contents of Wayland clipboard");

        std::thread::sleep(std::time::Duration::from_secs(5));

        assert_eq!(
            clipboard
                .get_contents()
                .expect("couldn't get contents of Wayland clipboard"),
            "foo bar baz"
        );
    }
}
