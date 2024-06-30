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

const ansi = @import("ansi.zig");
const serial = @import("serial.zig");

pub fn init() void {
    const writer = serial.Writer{ .context = .{} };
    try writer.print("{s}{s}NeurOS v0.1.0 (x86_64)\r\n{s}Copyright (C) 2024 Theomund{s}{s}\r\n\n", .{ ansi.bold, ansi.red, ansi.blue, ansi.normal, ansi.default });
}
