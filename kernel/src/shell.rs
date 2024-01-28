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

use crate::serial::SERIAL;

pub fn initialize() {
    let title = concat!(
        "\x1b[1;91m",
        "NeurOS v",
        env!("CARGO_PKG_VERSION"),
        " (x86_64)\n\r"
    );
    SERIAL.print(title);
    let copyright = concat!(
        "\x1b[1;94m",
        "Copyright (C) 2024 ",
        env!("CARGO_PKG_AUTHORS"),
        "\n\n\r"
    );
    SERIAL.print(copyright);
    SERIAL.print("\x1b[1;39m> ");
    loop {
        let character = SERIAL.read();
        match character {
            b'\r' => SERIAL.print("\n\r> "),
            _ => SERIAL.write(character),
        }
    }
}
