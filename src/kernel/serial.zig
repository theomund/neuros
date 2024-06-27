// NeurOS - Hobbyist operating system written in Zig.
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

const PORT = 0x3f8;

pub fn init() void {
    outb(PORT + 1, 0x00);
    outb(PORT + 3, 0x80);
    outb(PORT + 0, 0x03);
    outb(PORT + 1, 0x00);
    outb(PORT + 3, 0x03);
    outb(PORT + 2, 0xC7);
    outb(PORT + 4, 0x0B);
    outb(PORT + 4, 0x1E);
    outb(PORT + 0, 0xAE);

    if (inb(PORT + 0) != 0xAE) {
        @panic("Failed to initialize serial port.");
    }

    outb(PORT + 4, 0x0F);

    const message = "NeurOS v0.1.0 (x86_64)\r\nCopyright (C) 2024 Theomund";

    for (message) |character| {
        write(character);
    }
}

fn inb(address: u16) u8 {
    return asm volatile ("inb %[address], %[value]"
        : [value] "={al}" (-> u8),
        : [address] "N{dx}" (address),
    );
}

fn outb(address: u16, value: u8) void {
    asm volatile ("outb %[value], %[address]"
        :
        : [address] "{dx}" (address),
          [value] "{al}" (value),
    );
}

fn received() u8 {
    return inb(PORT + 5) & 1;
}

fn read() u8 {
    while (received() == 0) {}
    return inb(PORT);
}

fn transmit_empty() u8 {
    return inb(PORT + 5) & 0x20;
}

fn write(character: u8) void {
    while (transmit_empty() == 0) {}
    outb(PORT, character);
}
