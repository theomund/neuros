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

use alloc::string::ToString;
use core::fmt::{Arguments, Result, Write};
use spin::{Lazy, Mutex};
use x86_64::instructions::port::Port;

pub static SERIAL: Lazy<Mutex<Serial>> = Lazy::new(|| {
    let serial = Serial::new(Ports::COM1);
    serial.initialize();
    Mutex::new(serial)
});

pub struct Serial {
    address: u16,
}

pub enum Ports {
    COM1 = 0x3F8,
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> Result {
        for character in s.chars() {
            self.write_char(character)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result {
        while self.transmit_empty() == 0 {}
        self.outb(0, c as u8);
        Ok(())
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        self.write_str(args.to_string().as_str())?;
        Ok(())
    }
}

impl Serial {
    pub fn new(port: Ports) -> Serial {
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
        let mut port: Port<u8> = Port::new(self.address + offset);
        unsafe { port.read() }
    }

    fn outb(&self, offset: u16, value: u8) {
        let mut port: Port<u8> = Port::new(self.address + offset);
        unsafe {
            port.write(value);
        }
    }

    fn transmit_empty(&self) -> u8 {
        self.inb(5) & 0x20
    }

    fn received(&self) -> u8 {
        self.inb(5) & 0x1
    }

    pub fn read(&self) -> u8 {
        while self.received() == 0 {}
        self.inb(0)
    }
}
