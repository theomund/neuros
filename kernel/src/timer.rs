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

use core::sync::atomic::{AtomicU32, Ordering};
use spin::Lazy;

pub static TIMER: Lazy<Timer> = Lazy::new(Timer::new);

pub struct Timer {
    elapsed: AtomicU32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            elapsed: AtomicU32::new(0),
        }
    }

    pub fn get_elapsed(&self) -> u32 {
        self.elapsed.load(Ordering::Relaxed)
    }

    pub fn increment(&self) {
        self.elapsed.fetch_add(1, Ordering::Relaxed);
    }
}
