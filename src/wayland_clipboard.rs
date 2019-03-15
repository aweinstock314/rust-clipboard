/*
Copyright 2019 Avraham Weinstock

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

use smithay_clipboard::WaylandClipboard;
use wayland_client::Display;

use std::error::Error;

pub struct WaylandClipboardContext(WaylandClipboard, String);

impl WaylandClipboardContext {
    pub fn new(display: &Display) -> Result<Self, Box<Error>> {
        let clipboard = WaylandClipboard::new_threaded(display);
        Ok(WaylandClipboardContext(clipboard, "seat0".to_string()))
    }
    pub fn get_contents(&mut self) -> Result<String, Box<Error>> {
        Ok(self.0.load(self.1.clone()))
    }
    pub fn set_contents(&mut self, data: String) -> Result<(), Box<Error>> {
        Ok(self.0.store(self.1.clone(), data))
    }
    pub fn set_seat(&mut self, name: String) {
        self.1 = name
    }
}
