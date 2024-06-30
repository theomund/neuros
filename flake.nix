# NeurOS - Hobbyist operating system written in Zig.
# Copyright (C) 2024 Theomund
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

{
  description = "Hobbyist operating system written in Zig.";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; }
      {
        systems = [ "aarch64-linux" "x86_64-linux" ];
        perSystem = { lib, pkgs, system, ... }:
          with pkgs;
          {
            devShells.default = mkShell {
              packages = [
                OVMF.fd
                gdb
                git
                limine
                qemu
                vale
                xorriso
                zig
              ];

              shellHook = ''
                export LIMINE_DATA=${limine}/share/limine
                export OVMF_FD=${OVMF.fd}/FV/OVMF.fd
                echo "Welcome to the NeurOS development shell."
              '';
            };

            formatter = nixpkgs-fmt;
          };
      };
}
