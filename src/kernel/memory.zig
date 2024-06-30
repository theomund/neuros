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

const limine = @import("limine");
const serial = @import("serial.zig");
const std = @import("std");

const Log = std.log.scoped(.memory);

pub export var memory_map_request: limine.MemoryMapRequest = .{};

pub fn init() void {
    if (memory_map_request.response) |memory_map_response| {
        const count = memory_map_response.entry_count;
        Log.debug("Detected {d} entries in the memory map.", .{count});
        const entries = memory_map_response.entries();
        var usable: u32 = 0;
        for (entries) |entry| {
            Log.debug("Found memory map entry of type {s}.", .{@tagName(entry.kind)});
            if (entry.kind == limine.MemoryMapEntryType.usable) {
                usable += 1;
            }
        }
        Log.debug("Detected {d} usable memory map entries.", .{usable});
        Log.info("Initialized the memory subsystem.", .{});
    }
}
