// NeurOS - Hobbyist operating system written in Rust.
// Copyright (C) 2024 Theomund
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use spin::{Lazy, Mutex};

pub static TIMER: Lazy<Mutex<Timer>> = Lazy::new(|| {
    let timer = Timer::new();
    Mutex::new(timer)
});

pub struct Timer {
    elapsed: u32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { elapsed: 0 }
    }

    pub fn get_elapsed(&self) -> u32 {
        self.elapsed
    }

    pub fn set_elapsed(&mut self, value: u32) {
        self.elapsed = value
    }
}
