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
const std = @import("std");

pub export var module_request: limine.ModuleRequest = .{};

const Log = std.log.scoped(.initrd);

pub fn init() void {
    if (module_request.response) |module_response| {
        const initrd = module_response.modules()[0];
        Log.debug("Detected initial RAM disk module with {s} as its path.", .{initrd.path});
        Log.info("Initialized the initial RAM disk (initrd) subsystem.", .{});
    }
}
