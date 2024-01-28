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

use core::arch::asm;
use spin::Lazy;

pub static SERIAL: Lazy<Serial> = Lazy::new(|| {
    let serial = Serial::new(Port::COM1);
    serial.initialize();
    serial
});

pub struct Serial {
    address: u16,
}

pub enum Port {
    COM1 = 0x3F8,
}

impl Serial {
    pub fn new(port: Port) -> Serial {
        Serial {
            address: port as u16,
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

    fn inb(&self, offset: u16) -> u8 {
        let port = self.address + offset;
        let value: u8;
        unsafe {
            asm!("inb %dx, %al", in("dx") port, out("al") value, options(att_syntax));
        }
        value
    }

    fn outb(&self, offset: u16, value: u8) {
        let port = self.address + offset;
        unsafe {
            asm!("outb %al, %dx", in("al") value, in("dx") port, options(att_syntax));
        }
    }

    fn transmit_empty(&self) -> u8 {
        self.inb(5) & 0x20
    }

    pub fn write(&self, value: u8) {
        while self.transmit_empty() == 0 {}
        self.outb(0, value);
    }

    fn received(&self) -> u8 {
        self.inb(5) & 0x1
    }

    pub fn read(&self) -> u8 {
        while self.received() == 0 {}
        self.inb(0)
    }

    pub fn print(&self, message: &str) {
        for byte in message.as_bytes() {
            self.write(*byte);
        }
    }
}
