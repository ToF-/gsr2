{
  description = "Rust with formatting, linting and test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
      in
      {
        devShells.default = pkgs.mkShellNoCC {
          packages = [
            pkgs.rustup
            pkgs.clang
            pkgs.bashInteractive
            pkgs.sqlite
            pkgs.pkg-config
            pkgs.gtk4
            pkgs.pango
            pkgs.gdk-pixbuf
            pkgs.gobject-introspection
            pkgs.cairo
            pkgs.gob2
            pkgs.glib
            pkgs.gio-sharp
          ];
        };
      }
    );
}
