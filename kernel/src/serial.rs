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

use limine::request::HhdmRequest;

static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

pub struct Port {
    address: usize,
}

impl Port {
    pub fn new(address: usize) -> Port {
        if let Some(hhdm_response) = HHDM_REQUEST.get_response() {
            let location = address + hhdm_response.offset() as usize;
            Port { address: location }
        } else {
            panic!("Failed to construct serial port.");
        }
    }

    pub fn initialize(&self) {
        self.outb(1, 0x00);
        self.outb(3, 0x80);
        self.outb(0, 0x03);
        self.outb(1, 0x00);
        self.outb(3, 0x03);
        self.outb(2, 0xC7);
        self.outb(4, 0x0B);
        self.outb(4, 0x1E);
        self.outb(0, 0xAE);

        if self.inb(0) != 0xAE {
            panic!("Failed to initialize serial port.");
        }

        self.outb(4, 0x0F);
    }

    fn inb(&self, offset: usize) -> usize {
        let location = (self.address + offset) as *mut usize;
        unsafe { *(location) }
    }

    fn outb(&self, offset: usize, value: usize) {
        let location = (self.address + offset) as *mut usize;
        unsafe {
            *(location) = value;
        }
    }
}
