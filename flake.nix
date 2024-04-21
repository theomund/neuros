{
  description = "Hobbyist operating system written in Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          gcc
          gdb
          git
          gnumake
          hadolint
          qemu
          OVMF
          podman
          rustup
          vale
          xorriso
        ];

        shellHook = ''
          echo "Welcome to the NeurOS development shell."
        '';
      };
      formatter.${system} = pkgs.nixpkgs-fmt;
    };
}
