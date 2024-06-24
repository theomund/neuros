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

const std = @import("std");

pub fn build(b: *std.Build) void {
    var target: std.zig.CrossTarget = .{
        .cpu_arch = .x86_64,
        .os_tag = .freestanding,
        .abi = .none,
    };

    const features = std.Target.x86.Feature;
    target.cpu_features_sub.addFeature(@intFromEnum(features.mmx));
    target.cpu_features_sub.addFeature(@intFromEnum(features.sse));
    target.cpu_features_sub.addFeature(@intFromEnum(features.sse2));
    target.cpu_features_sub.addFeature(@intFromEnum(features.avx));
    target.cpu_features_sub.addFeature(@intFromEnum(features.avx2));
    target.cpu_features_add.addFeature(@intFromEnum(features.soft_float));

    const optimize = b.standardOptimizeOption(.{});

    const limine = b.dependency("limine", .{});

    const kernel = b.addExecutable(.{
        .name = "kernel",
        .root_source_file = b.path("src/kernel/main.zig"),
        .target = b.resolveTargetQuery(target),
        .optimize = optimize,
        .code_model = .kernel,
        .pic = true,
    });
    kernel.root_module.addImport("limine", limine.module("limine"));
    kernel.setLinkerScriptPath(b.path("src/kernel/linker.ld"));
    kernel.want_lto = false;

    b.installArtifact(kernel);

    const kernel_step = b.step("kernel", "Build the kernel");
    kernel_step.dependOn(&kernel.step);

    const iso_step = b.step("iso", "Build the ISO");
    iso_step.dependOn(kernel_step);

    const run_step = b.step("run", "Run the operating system");
    run_step.dependOn(iso_step);
}
